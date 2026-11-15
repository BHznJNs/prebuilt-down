use anyhow::{Context, Result, anyhow};
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

#[derive(Default)]
pub struct Environment {
    overrides: HashMap<String, String>,
}

impl Environment {
    pub fn load(env_file: Option<&Path>) -> Result<Self> {
        let path = env_file.unwrap_or_else(|| Path::new(".env"));
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                tracing::warn!(
                    "Env file not found: {}; using process environment variables",
                    path.display()
                );
                return Ok(Self::default());
            }
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to read env file: {}", path.display()));
            }
        };

        let mut overrides = HashMap::new();
        for entry in dotenvy::from_read_iter(file) {
            // dotenvy parse errors include the source line, which can contain secrets.
            let (name, value) =
                entry.map_err(|_| anyhow!("failed to parse env file: {}", path.display()))?;
            overrides.insert(name, value);
        }
        return Ok(Self { overrides });
    }

    pub fn resolve(&self, template: &str) -> Result<String> {
        Ok(subst::substitute(template, self)?)
    }
}

impl<'a> subst::VariableMap<'a> for Environment {
    type Value = Cow<'a, str>;

    fn get(&'a self, name: &str) -> Option<Self::Value> {
        self.overrides
            .get(name)
            .map(|value| Cow::Borrowed(value.as_str()))
            .or_else(|| env::var(name).ok().map(Cow::Owned))
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;

    #[test]
    fn resolves_multiple_variables_without_recursively_expanding_values() {
        let environment = Environment {
            overrides: [
                ("TOKEN".into(), "token-${LITERAL}".into()),
                ("SUFFIX".into(), "tail".into()),
            ]
            .into(),
        };
        assert_eq!(
            environment.resolve("Bearer ${TOKEN}:${SUFFIX}!").unwrap(),
            "Bearer token-${LITERAL}:tail!"
        );
    }

    #[test]
    fn preserves_literal_values_and_escaped_dollar_signs() {
        assert_eq!(
            Environment::default()
                .resolve(r"Bearer literal-\$token")
                .unwrap(),
            "Bearer literal-$token"
        );
    }

    #[test]
    fn supports_unbraced_variables_and_default_values() {
        let environment = Environment {
            overrides: [("TOKEN".into(), "token".into())].into(),
        };
        assert_eq!(
            environment.resolve("Bearer $TOKEN").unwrap(),
            "Bearer token"
        );
        let missing = (0..)
            .map(|index| format!("PREBUILT_DOWN_TEST_UNSET_{index}"))
            .find(|name| std::env::var_os(name).is_none())
            .unwrap();
        assert_eq!(
            environment
                .resolve(&format!("${{{missing}:fallback}}"))
                .unwrap(),
            "fallback"
        );
        let error = environment
            .resolve(&format!("Bearer ${{{missing}}}"))
            .unwrap_err();
        assert!(error.to_string().contains(&missing));
    }

    #[test]
    fn rejects_unclosed_or_empty_placeholders() {
        let environment = Environment::default();
        assert!(environment.resolve("Bearer ${TOKEN").is_err());
        assert!(environment.resolve("Bearer ${}").is_err());
    }
}
