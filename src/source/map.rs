use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub expression: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionMap {
    pub expression: Range,
    pub begin: Option<Range>,
    pub end: Option<Range>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct OperatorMap {
    pub expression: Range,
    pub operator: Option<Range>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SendMap {
    pub expression: Range,
    pub dot: Option<Range>,
    pub selector: Option<Range>,
    pub operator: Option<Range>,
    pub begin: Option<Range>,
    pub end: Option<Range>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableMap {
    pub expression: Range,
    pub name: Option<Range>,
    pub operator: Option<Range>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeywordMap {
    pub expression: Range,
    pub keyword: Range,
    pub begin: Option<Range>,
    pub end: Option<Range>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionMap {
    pub expression: Range,
    pub keyword: Option<Range>,
    pub begin: Option<Range>,
    pub end: Option<Range>,
    pub else_: Option<Range>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDefinitionMap {
    pub keyword: Range,
    pub operator: Option<Range>,
    pub name: Range,
    pub end: Option<Range>,
    pub assignment: Option<Range>,
    pub expression: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantMap {
    pub double_colon: Option<Range>,
    pub name: Range,
    pub operator: Option<Range>,
    pub expression: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexMap {
    pub begin: Range,
    pub end: Range,
    pub operator: Option<Range>,
    pub expression: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RescueBodyMap {
    pub expression: Range,
    pub keyword: Range,
    pub assoc: Option<Range>,
    pub begin: Option<Range>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpAssignMap {
    pub expression: Range,
    pub operator: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionMap {
    pub expression: Range,
    pub keyword: Range,
    pub operator: Option<Range>,
    pub name: Option<Range>,
    pub end: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TernaryMap {
    pub expression: Range,
    pub question: Range,
    pub colon: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForMap {
    pub expression: Range,
    pub keyword: Range,
    pub in_: Range,
    pub begin: Range,
    pub end: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeredocMap {
    pub heredoc_body: Range,
    pub heredoc_end: Range,
    pub expression: Range,
}
