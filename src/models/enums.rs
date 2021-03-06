#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, serde::Deserialize, PartialEq)]
pub enum GroupID {
    /// GroupID represents a two-letter identifier for a pay group as an enum
    CS,
    CX,
    DS,
    LS,
    EC,
    EL,
    FB,
    FI,
    FS,
    AS,
    CM,
    CR,
    IS,
    PM,
    WP,
    HR,
    RO,
    DE,
    OP,
    PH,
    PS,
    VM,
    AC,
    AG,
    BI,
    CH,
    FO,
    PC,
    FR,
    LI,
    PR,
    SC,
    DD,
    EG,
    GT,
    PI,
    PY,
    TI,
    TR,
    UT,
}

#[derive(juniper::GraphQLEnum, Clone, Debug, serde::Deserialize, PartialEq)]
pub enum Period {
    /// Period represents a period of time in and contains the salary for that period.
    Annual,
    Weekly,
    Daily,
    Hourly,
}
