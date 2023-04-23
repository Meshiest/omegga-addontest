use napi::{
  Env, JsBoolean, JsFunction, JsNumber, JsObject, JsString, JsUnknown, Result, ValueType,
};

#[derive(Clone)]
pub struct Player {
  pub name: String,
  pub id: String,
  pub controller: String,
  pub state: String,
}

#[derive(Clone)]
pub struct UserId {
  pub name: String,
  pub id: String,
}

pub struct Omegga {
  inner: JsObject,
  env: Env,
}

#[derive(Debug)]
pub enum OmeggaError {
  MissingField(&'static str),
  InvalidFieldType {
    name: &'static str,
    found: ValueType,
    expected: ValueType,
  },
}

impl Omegga {
  pub fn new(obj: JsObject, env: Env) -> std::result::Result<Self, OmeggaError> {
    let fields = [
      ("version", ValueType::Number),
      ("verbose", ValueType::Boolean),
      ("players", ValueType::Object),
      ("host", ValueType::Object),
      ("started", ValueType::Boolean),
      ("starting", ValueType::Boolean),
      ("stopping", ValueType::Boolean),
      ("currentMap", ValueType::String),
      ("configPath", ValueType::String),
      ("savePath", ValueType::String),
      ("presetPath", ValueType::String),
      ("writeln", ValueType::Function),
    ];

    for (name, expected_type) in fields {
      let value = obj
        .get_named_property::<JsUnknown>(name)
        .map_err(|_| OmeggaError::MissingField(name))?;
      let value_type = value
        .get_type()
        .map_err(|_| OmeggaError::MissingField(name))?;
      if value_type != expected_type {
        Err(OmeggaError::InvalidFieldType {
          name,
          found: value_type,
          expected: expected_type,
        })?
      }
    }

    Ok(Omegga { inner: obj, env })
  }

  fn get_bool(&self, name: &'static str) -> bool {
    self
      .inner
      .get_named_property::<JsBoolean>(name)
      .and_then(|b| b.get_value())
      .unwrap_or_default()
  }

  fn get_string(&self, name: &'static str) -> Result<String> {
    let string = self
      .inner
      .get_named_property::<JsString>(name)?
      .into_utf8()?
      .as_str()?
      .to_string();

    Ok(string)
  }

  pub fn get_version(&self) -> i32 {
    self
      .inner
      .get_named_property::<JsNumber>("version")
      .and_then(|n| n.get_int32())
      .unwrap_or(-1)
  }

  pub fn get_verbose(&self) -> bool {
    self.get_bool("verbose")
  }

  pub fn get_started(&self) -> bool {
    self.get_bool("started")
  }

  pub fn get_stopping(&self) -> bool {
    self.get_bool("stopping")
  }

  pub fn get_map(&self) -> String {
    self.get_string("currentMap").unwrap()
  }

  pub fn get_config_path(&self) -> String {
    self.get_string("configPath").unwrap()
  }

  pub fn get_save_path(&self) -> String {
    self.get_string("savePath").unwrap()
  }

  pub fn get_preset_path(&self) -> String {
    self.get_string("presetPath").unwrap()
  }

  pub fn writeln(&self, line: &str) {
    if let Ok(func) = self.inner.get_named_property::<JsFunction>("writeln") {
      let _ = func.call::<JsUnknown>(
        Some(&self.inner),
        &[self.env.create_string(line).unwrap().into_unknown()],
      );
    }
  }
}
