use convert_case::{Case, Casing};
use homm5_types::region::{Region, Regions};
use quick_xml::Writer;

pub struct RegionModifier {
    regions: Vec<String>,
}

impl RegionModifier {
    pub fn new() -> Self {
        RegionModifier { regions: Vec::new() }
    }

    pub fn modify(&mut self, regions: &mut Regions, writer: &mut Writer<&mut Vec<u8>>) {
        if let Some(items) = &regions.items {
            for region in items {
                self.collect(region);
            }
        }
        writer.write_serializable("regions", regions).unwrap();
    }

    fn collect(&mut self, region: &Region) {
        if region.name.starts_with("region_") && !self.regions.contains(&region.name) {
            self.regions.push(region.name.clone());
        }
    }

    pub fn convert_to_lua(&self) -> String {
        let mut lua_code = String::from("---@enum Regions\nRegions = {\n");
        for region in &self.regions {
            let name = region
                .strip_prefix("region_")
                .unwrap_or(region)
                .to_case(Case::Pascal);
            if name.is_empty() {
                continue;
            }
            lua_code += &format!("\t{} = \"{}\",\n", name, region);
        }
        lua_code.push_str("}\n\n__end_import()");
        lua_code
    }
}

impl Default for RegionModifier {
    fn default() -> Self {
        Self::new()
    }
}
