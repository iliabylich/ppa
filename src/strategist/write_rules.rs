use crate::{
    action::macros::{cmd, write_file},
    config::Rules,
    plan::Plan,
};
use std::collections::HashMap;

pub(crate) fn write_rules(plan: &mut Plan, build_dir: &str, mut rules: Rules) {
    plan.push(cmd!("mkdir", "-p", format!("{build_dir}/debian")));

    // let contents = templates.rules(targets)?;
    let Some(rules) = rules.map.take() else {
        return;
    };

    let targets = compile_targets(rules);
    plan.push(write_file!(
        format!("{build_dir}/debian/rules"),
        "#!/usr/bin/make -f
export DH_VERBOSE = 1

{targets}
"
    ));
    plan.push(cmd!("chmod", "+x", format!("{build_dir}/debian/rules")));
}

fn compile_targets(mut targets: HashMap<String, Vec<String>>) -> String {
    let mut out = vec![];
    if let Some((target, lines)) = targets.remove_entry("%") {
        out.push(format!("{target}:"));
        for line in lines {
            out.push(format!("\t{line}"));
        }
    }

    out.push("override_dh_auto_test:".to_string());
    out.push("\t@true".to_string());

    out.push("override_dh_strip:".to_string());
    out.push("\t@true".to_string());

    for (target, lines) in targets {
        out.push(format!("{target}:"));
        for line in lines {
            out.push(format!("\t{line}"));
        }
    }
    out.join("\n")
}
