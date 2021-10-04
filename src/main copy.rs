
#[derive(Clone)]
struct Weapon {
    id: String,
    name: String,
    effects: Vec<Effect>,
}
impl Weapon {
    pub fn add(&self, effect: &Effect) -> Result<Weapon, Weapon> {
        let mut result = self.clone();
        if result.effects.contains(effect) {
            return Err(result.add(&Effect::Intense).map_or_else(|x| x, |x| x));
        }
        result.effects.push(effect.clone());
        Ok(result)
    }
    pub fn to_string(&mut self) -> String {
        let mut result = self.id.clone();
        self.effects.sort();
        for line in &self.effects{
            result.push_str(&("_".to_string() + &line.to_string()));
        }
        result
    }
    pub fn name(&mut self) -> String {
        let mut result = self.name.clone();
        self.effects.sort();
        for line in &self.effects {
            result = line.name() + " " + &result;
        }
        result
    }
    pub fn lowercase_id(&mut self) -> String {
        let mut result = self.name.clone();
        self.effects.sort();
        for line in &self.effects {
            result.push('_');
            result.push_str(&line.name());
        }
        result
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Effect {
    Ion,
    Pierce,
    Fire,
    Bio,
    Stun,
    Intense,
}
impl Effect {
    pub fn to_string(&self) -> String {
        match self {
            Effect::Bio => "BIO".to_string(),
            Effect::Fire => "FIRE".to_string(),
            Effect::Ion => "ION".to_string(),
            Effect::Pierce => "PIERCE".to_string(),
            Effect::Stun => "STUN".to_string(),
            Effect::Intense => "2".to_string(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Effect::Bio => "bio".to_string(),
            Effect::Fire => "fire".to_string(),
            Effect::Ion => "ion".to_string(),
            Effect::Pierce => "pierce".to_string(),
            Effect::Stun => "stun".to_string(),
            Effect::Intense => "intense".to_string(),
        }
    }
}
struct WeaponStructure {
    weapon: Weapon,
    possible_effects: Vec<Effect>
}
impl WeaponStructure {
    pub fn crafting_event_2(&mut self, double_allowed: bool) -> String {
        let mut result:String = String::new();
        result.push_str(&self.crafting_event(double_allowed));
        for line in self.possible_effects.clone() {
            result.push_str(&WeaponStructure {
                possible_effects:self.possible_effects.clone(),
                weapon: self.weapon.add(&line).unwrap_or_else(|x| x)
            }.crafting_event(double_allowed));
        }
        result
    }
    pub fn crafting_event(&mut self, double_allowed: bool) -> String {
        let mut result = format!("    <choice req=\"{weapon}\">
        <text> Craft your {name} (note: You will get it back if you decide not to modify it) </text>
        <event>
            <text> You put your {name} on the table... </text>
            <remove name=\"{weapon}\"/>
            <choice hidden=\"true\">
                <text> Never mind - do something else </text>
                <event>
                    <weapon name=\"{weapon}\"/>
                    <choice hidden=\"true\">
                        <text> Continue... </text>
                        <event load=\"CRAFTING_LOAD\"/>
                    </choice>
                </event>
            </choice>", weapon=self.weapon.to_string(), name=self.weapon.name());
        for effect in &self.possible_effects {
            let new = self.weapon.add(effect);
            if let Ok(mut val) = new {
                result.push_str(&format!("
            <choice req=\"EFFECT_{id}\">
                <text>({name} effect) Add the {name} effect to the {w_name} (will increase charge time)</text>
                <event>
                    <text> You combine the two weapons. </text>
                    <remove name=\"EFFECT_{id}\"/>
                    <choice hidden=\"true\">
                        <text> Pick up the result </text>
                        <event>
                            <text> You pick up the resulting weapon. </text>
                            <weapon name=\"{new}\"/>
                            <event load=\"CRAFTING_LOAD\"/>
                        </event>
                    </choice>
                </event>
            </choice>", id=effect.to_string(), new=val.to_string(), name=effect.name(), w_name = self.weapon.name()));
            } else if let Err(mut val) = new {
                if double_allowed {
                    result.push_str(&format!("
            <choice req=\"EFFECT_{id}\">
                <text>({name} effect) Add the {name} effect to the {w_name} (will increase charge time)</text>
                <event>
                    <text> You combine the two weapons. </text>
                    <remove name=\"EFFECT_{id}\"/>
                    <choice hidden=\"true\">
                        <text> Pick up the result </text>
                        <event>
                            <text> You pick up the resulting weapon. </text>
                            <weapon name=\"{new}\"/>
                            <event load=\"CRAFTING_LOAD\"/>
                        </event>
                    </choice>
                </event>
            </choice>", id=effect.to_string(), new=val.to_string(), name=effect.name(), w_name = self.weapon.name()));
                }
            }
        };
        result.push_str(
            "
        </event>
    </choice>
");
        result
    }
}
fn main() {
    let mut missiles:String = String::new();
    let mut structure: WeaponStructure = WeaponStructure {
        weapon: Weapon {
            id: "MISSILES_1".to_string(),
            effects: Vec::new(),
            name: "Leto missile".to_string()
        },
        possible_effects: vec![
            Effect::Bio,
            Effect::Fire,
            Effect::Ion,
            Effect::Pierce,
            Effect::Stun
        ] // all effects
    };
        missiles.push_str(&structure.crafting_event(false));

        structure.weapon = Weapon {
            id: "MISSILES_2".to_string(),
            effects:Vec::new(),
            name: "Artemis missile".to_string()
        };
        missiles.push_str(&structure.crafting_event(false));
        structure.weapon = Weapon {
            id: "MISSILES_3".to_string(),
            effects:Vec::new(),
            name: "Hermes missile".to_string()
        };
        missiles.push_str(&structure.crafting_event(false));
        structure.weapon = Weapon {
            id: "MISSILES_BURST".to_string(),
            effects:Vec::new(),
            name: "Pegasus missile".to_string()
        };
        missiles.push_str(&structure.crafting_event(false));
        structure.weapon = Weapon {
            id: "MISSILES_DISPENSER".to_string(),
            effects:Vec::new(),
            name: "Dispenser missile".to_string()
        };
        missiles.push_str(&structure.crafting_event_2(true));

    let mut lasers:String = String::new();
    let mut restricted_structure: WeaponStructure = WeaponStructure {
        weapon: Weapon {
            id: "MISSILES_1".to_string(),
            effects: Vec::new(),
            name: "Leto missile".to_string()
        },
        possible_effects: vec![
            Effect::Bio,
            Effect::Fire,
            Effect::Pierce,
            Effect::Stun
        ] // all effects
    };
        restricted_structure.weapon = Weapon {
            id: "LASER_BURST_1".to_string(),
            effects:Vec::new(),
            name: "Basic laser".to_string()
        };
        lasers.push_str(&restricted_structure.crafting_event_2(false));
        restricted_structure.weapon = Weapon {
            id: "LASER_BURST_2".to_string(),
            effects:Vec::new(),
            name: "Basic laser".to_string()
        };
        lasers.push_str(&restricted_structure.crafting_event_2(false));
        restricted_structure.weapon = Weapon {
            id: "LASER_BURST_3".to_string(),
            effects:Vec::new(),
            name: "Basic laser".to_string()
        };
        lasers.push_str(&restricted_structure.crafting_event_2(false));
        restricted_structure.weapon = Weapon {
            id: "LASER_BURST_5".to_string(),
            effects:Vec::new(),
            name: "Basic laser".to_string()
        };
        lasers.push_str(&restricted_structure.crafting_event_2(false));
        restricted_structure.weapon = Weapon {
            id: "LASER_DISPENSER".to_string(),
            effects:Vec::new(),
            name: "Laser dispenser".to_string()
        };
        lasers.push_str(&restricted_structure.crafting_event_2(true));

    let mut output = lasers;
    output.push_str(&missiles);
    std::fs::write("output", output).expect("FAILED");
}