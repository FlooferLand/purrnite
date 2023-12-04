// - Skin info -
// Note: The program isn't doing an exact match search.
//       Each tag and info piece will be searched individually,
//       If even one of them match it will trigger
pub const SKINS: [SkinDef; 3] = [
    SkinDef {
        name: &["Raven Team Leader", "Raven Leader", "Reaven Team Leader"],
        id: &["ScareyBeary"],
        tag: SkinTag::RavenTeamLeader,
        set: Some(SkinSet {
            id: "NotLove",
            name: "Nevermore Hearts Pack"
        })
    },
    SkinDef {
        name: &["Highwire", "High Wire", "High-Wire", "Pack Leader Highwire"],
        id: &["VitalPsych"],
        tag: SkinTag::PackLeaderHighwire,
        set: Some(SkinSet {
            id: "AnarchyExpert",
            name: "Chaos Artist"
        })
    },
    SkinDef {
        name: &["Meow Skulls", "Meowskulls", "Chill cat"],
        id: &["ChillCat"],
        tag: SkinTag::Meowskulls,
        set: Some(SkinSet {
            id: "Cali-Cool",
            name: "Cali-Cool"
        })
    }
];

#[derive(Debug)]
pub enum SkinTag {
    RavenTeamLeader,
    PackLeaderHighwire,
    Meowskulls
}

pub struct SkinDef<'a> {
    pub name: &'a [&'a str],
    pub tag: SkinTag,
    pub id: &'a [&'a str],
    pub set: Option<SkinSet<'a>>
}

pub struct SkinSet<'a> {
    pub id: &'a str,
    pub name: &'a str
}

