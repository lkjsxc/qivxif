use crate::error::{QualityError, finish};
use std::{collections::BTreeSet, fs, path::Path};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Route {
    method: String,
    path: String,
}

pub fn check_route_docs_match_api() -> Result<(), QualityError> {
    let docs = fs::read_to_string("docs/architecture/server/axum-api.md")?;
    let documented = documented_routes(&docs);
    let mounted = mounted_routes(Path::new("apps/qivxif-server/src/routes"))?;
    let mut failures = Vec::new();
    for route in documented.difference(&mounted) {
        failures.push(format!(
            "documented route is not mounted: {}",
            route.label()
        ));
    }
    for route in mounted.difference(&documented) {
        failures.push(format!(
            "mounted route is not documented: {}",
            route.label()
        ));
    }
    finish(failures, "routes match docs")
}

fn documented_routes(contents: &str) -> BTreeSet<Route> {
    contents.lines().filter_map(documented_route).collect()
}

fn documented_route(line: &str) -> Option<Route> {
    let cells: Vec<_> = line.split('|').map(str::trim).collect();
    if cells.len() < 4 {
        return None;
    }
    let method = code_cell(cells[1])?;
    let path = code_cell(cells[2])?;
    (path.starts_with('/')).then_some(Route { method, path })
}

fn mounted_routes(dir: &Path) -> Result<BTreeSet<Route>, QualityError> {
    let mut routes = BTreeSet::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let contents = fs::read_to_string(path)?;
        routes.extend(contents.lines().filter_map(mounted_route));
    }
    Ok(routes)
}

fn mounted_route(line: &str) -> Option<Route> {
    let marker = ".route(\"";
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    let path = rest[..end].to_owned();
    let tail = &rest[end..];
    let method = if tail.contains("get(") {
        "GET"
    } else if tail.contains("post(") {
        "POST"
    } else {
        return None;
    };
    Some(Route {
        method: method.to_owned(),
        path,
    })
}

fn code_cell(cell: &str) -> Option<String> {
    let start = cell.find('`')? + 1;
    let end = cell[start..].find('`')? + start;
    Some(cell[start..end].to_owned())
}

impl Route {
    fn label(&self) -> String {
        format!("{} {}", self.method, self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_documented_route_table_row() {
        let route = documented_route("| `GET` | `/health` | public | no |").unwrap();
        assert_eq!(route.label(), "GET /health");
    }

    #[test]
    fn parses_mounted_route_declaration() {
        let route = mounted_route(r#"Router::new().route("/api/me", get(me))"#).unwrap();
        assert_eq!(route.label(), "GET /api/me");
    }
}
