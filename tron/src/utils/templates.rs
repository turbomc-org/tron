use std::collections::HashMap;
use text_placeholder::Template;

pub fn render_template(t: &Template<'static>, vars: &HashMap<&str, &str>) -> String {
    t.fill_with_hashmap(vars)
}

#[macro_export]
macro_rules! render {
    ($tmpl:ident, $($key:ident = $val:expr),+ $(,)?) => {{
        use std::collections::HashMap;
        let mut __vars: HashMap<&str, String> = HashMap::new();
        $(
            __vars.insert(stringify!($key), $val.to_string());
        )+
        let ref_map: HashMap<&str, &str> =
            __vars.iter().map(|(k, v)| (*k, v.as_str())).collect();
        $crate::render_template(&$tmpl, &ref_map)
    }};
    ($tmpl:ident, $vars:expr) => {{
        $crate::render_template(&$tmpl, $vars)
    }};
}
