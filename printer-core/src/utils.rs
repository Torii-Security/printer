use syn::Stmt;

#[derive(Clone, Debug)]
pub struct HooksData {
    pub(crate) before_each: Vec<Vec<Stmt>>,
    pub(crate) after_each: Vec<Vec<Stmt>>,
    pub(crate) invariants: Vec<Vec<Stmt>>,
    pub(crate) before_all: Vec<Stmt>, // potentially other vecs can also look like this
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrinterAttributeVariant {
    BeforeEach,
    AfterEach,
    Invariant,
    BeforeAll,
}

impl PrinterAttributeVariant {
    pub fn try_from_string(value: String) -> Option<Self> {
        if value == "before_each" {
            return Some(Self::BeforeEach);
        } else if value == "after_each" {
            return Some(Self::AfterEach);
        } else if value == "invariant" {
            return Some(Self::Invariant);
        } else if value == "before_all" {
            return Some(Self::BeforeAll);
        }

        None
    }

    pub fn populate_hooks_data(&self, hooks_data: &mut HooksData, stmt_vec: Vec<Stmt>) {
        match self {
            PrinterAttributeVariant::BeforeEach => hooks_data.before_each.push(stmt_vec),
            PrinterAttributeVariant::AfterEach => hooks_data.after_each.push(stmt_vec),
            PrinterAttributeVariant::Invariant => hooks_data.invariants.push(stmt_vec),
            PrinterAttributeVariant::BeforeAll => hooks_data.before_all.extend(stmt_vec),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionAttributeVariant {
    Printer(PrinterAttributeVariant),
    External,
}

impl From<String> for FunctionAttributeVariant {
    fn from(value: String) -> Self {
        if let Some(variant) = PrinterAttributeVariant::try_from_string(value) {
            return Self::Printer(variant);
        }

        Self::External
    }
}
