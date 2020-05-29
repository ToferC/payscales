pub struct Query;
pub mod query {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "Query";
    pub const QUERY : & 'static str = "query Query(\n  $identifier1:GroupID!,\n  $level: Int!, \n  $step:Int!,\n\t$startDate: NaiveDate!,\n\t$endDate:NaiveDate!) {\n  group(identifier: $identifier1) {\n    payscaleForLevel(level:$level) {\n      steps\n    }\n    identifier\n    payAtLevelAndStepBetweenDates(\n    \tlevel: $level\n      step: $step\n      startDate:$startDate\n      endDate:$endDate\n    ){\n      startDate\n      endDate\n      workDays\n      workHours\n      hourlyRate\n      annualRate\n      salary\n    }\n  }\n}" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type NaiveDate = super::NaiveDate;
    #[derive(Eq, PartialEq, Clone)]
    pub enum GroupID {
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
        Other(String),
    }
    impl ::serde::Serialize for GroupID {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                GroupID::CS => "CS",
                GroupID::CX => "CX",
                GroupID::DS => "DS",
                GroupID::LS => "LS",
                GroupID::EC => "EC",
                GroupID::EL => "EL",
                GroupID::FB => "FB",
                GroupID::FI => "FI",
                GroupID::FS => "FS",
                GroupID::AS => "AS",
                GroupID::CM => "CM",
                GroupID::CR => "CR",
                GroupID::IS => "IS",
                GroupID::PM => "PM",
                GroupID::WP => "WP",
                GroupID::HR => "HR",
                GroupID::RO => "RO",
                GroupID::DE => "DE",
                GroupID::OP => "OP",
                GroupID::PH => "PH",
                GroupID::PS => "PS",
                GroupID::VM => "VM",
                GroupID::AC => "AC",
                GroupID::AG => "AG",
                GroupID::BI => "BI",
                GroupID::CH => "CH",
                GroupID::FO => "FO",
                GroupID::PC => "PC",
                GroupID::FR => "FR",
                GroupID::LI => "LI",
                GroupID::PR => "PR",
                GroupID::SC => "SC",
                GroupID::DD => "DD",
                GroupID::EG => "EG",
                GroupID::GT => "GT",
                GroupID::PI => "PI",
                GroupID::PY => "PY",
                GroupID::TI => "TI",
                GroupID::TR => "TR",
                GroupID::UT => "UT",
                GroupID::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GroupID {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CS" => Ok(GroupID::CS),
                "CX" => Ok(GroupID::CX),
                "DS" => Ok(GroupID::DS),
                "LS" => Ok(GroupID::LS),
                "EC" => Ok(GroupID::EC),
                "EL" => Ok(GroupID::EL),
                "FB" => Ok(GroupID::FB),
                "FI" => Ok(GroupID::FI),
                "FS" => Ok(GroupID::FS),
                "AS" => Ok(GroupID::AS),
                "CM" => Ok(GroupID::CM),
                "CR" => Ok(GroupID::CR),
                "IS" => Ok(GroupID::IS),
                "PM" => Ok(GroupID::PM),
                "WP" => Ok(GroupID::WP),
                "HR" => Ok(GroupID::HR),
                "RO" => Ok(GroupID::RO),
                "DE" => Ok(GroupID::DE),
                "OP" => Ok(GroupID::OP),
                "PH" => Ok(GroupID::PH),
                "PS" => Ok(GroupID::PS),
                "VM" => Ok(GroupID::VM),
                "AC" => Ok(GroupID::AC),
                "AG" => Ok(GroupID::AG),
                "BI" => Ok(GroupID::BI),
                "CH" => Ok(GroupID::CH),
                "FO" => Ok(GroupID::FO),
                "PC" => Ok(GroupID::PC),
                "FR" => Ok(GroupID::FR),
                "LI" => Ok(GroupID::LI),
                "PR" => Ok(GroupID::PR),
                "SC" => Ok(GroupID::SC),
                "DD" => Ok(GroupID::DD),
                "EG" => Ok(GroupID::EG),
                "GT" => Ok(GroupID::GT),
                "PI" => Ok(GroupID::PI),
                "PY" => Ok(GroupID::PY),
                "TI" => Ok(GroupID::TI),
                "TR" => Ok(GroupID::TR),
                "UT" => Ok(GroupID::UT),
                _ => Ok(GroupID::Other(s)),
            }
        }
    }
    #[derive(Deserialize)]
    pub struct QueryGroupPayscaleForLevel {
        pub steps: Int,
    }
    #[derive(Deserialize)]
    pub struct QueryGroupPayAtLevelAndStepBetweenDates {
        #[serde(rename = "startDate")]
        pub start_date: NaiveDate,
        #[serde(rename = "endDate")]
        pub end_date: NaiveDate,
        #[serde(rename = "workDays")]
        pub work_days: Float,
        #[serde(rename = "workHours")]
        pub work_hours: Float,
        #[serde(rename = "hourlyRate")]
        pub hourly_rate: Float,
        #[serde(rename = "annualRate")]
        pub annual_rate: Float,
        pub salary: Option<Float>,
    }
    #[derive(Deserialize)]
    pub struct QueryGroup {
        #[serde(rename = "payscaleForLevel")]
        pub payscale_for_level: Option<QueryGroupPayscaleForLevel>,
        pub identifier: GroupID,
        #[serde(rename = "payAtLevelAndStepBetweenDates")]
        pub pay_at_level_and_step_between_dates:
            Option<Vec<QueryGroupPayAtLevelAndStepBetweenDates>>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub identifier1: GroupID,
        pub level: Int,
        pub step: Int,
        #[serde(rename = "startDate")]
        pub start_date: NaiveDate,
        #[serde(rename = "endDate")]
        pub end_date: NaiveDate,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub group: QueryGroup,
    }
}
impl graphql_client::GraphQLQuery for Query {
    type Variables = query::Variables;
    type ResponseData = query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: query::QUERY,
            operation_name: query::OPERATION_NAME,
        }
    }
}
