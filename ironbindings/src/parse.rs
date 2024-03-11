use rustdoc_types::{Id, Item, Visibility};

pub fn parse_crate(krate: &rustdoc_types::Crate) -> Module {
    let module_item = krate.index.get(&krate.root).unwrap();
    let module = match &module_item.inner {
        rustdoc_types::ItemEnum::Module(m) => m,
        _ => unreachable!("Expected module item"),
    };
    Module::parse(krate, &module_item, &module)
}

#[derive(Debug, Clone)]
pub struct Module {
    pub id: Id,
    pub path: Vec<String>,
    pub name: String,
    pub doc: Option<String>,
    pub visibility: Visibility,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub modules: Vec<Module>,
}

impl Module {
    pub fn parse(krate: &rustdoc_types::Crate, module_item: &Item, module: &rustdoc_types::Module) -> Module {
        std::thread::scope(|scope| {
            let mut structs_handles = Vec::new();
            let mut functions_handles = Vec::new();
            let mut modules_handles = Vec::new();

            for id in module.items.iter() {
                let item = krate.index.get(id).unwrap();
                // skip private things
                if item.visibility != Visibility::Public {
                    continue;
                }
                // skip hidden things
                if item.attrs.iter().any(|attr| attr.to_string() == "doc(hidden)") {
                    continue;
                }
                match &item.inner {
                    rustdoc_types::ItemEnum::Struct(s) => {
                        structs_handles.push(scope.spawn(|| Struct::parse(krate, item, s)));
                    }
                    rustdoc_types::ItemEnum::Function(f) => {
                        functions_handles.push(scope.spawn(|| Function::parse(krate, item, f)));
                    }
                    rustdoc_types::ItemEnum::Module(m) => {
                        modules_handles.push(scope.spawn(|| Module::parse(krate, item, m)));
                    }
                    // TODO!: handle re-exports
                    // TODO!: handle globals
                    _ => {}
                }
            }

            Module {
                id: module_item.id.clone(),
                path: krate.paths.get(&module_item.id).unwrap().path.iter().cloned().collect(),
                name: module_item.name.clone().unwrap(),
                visibility: module_item.visibility.clone(),
                doc: module_item.docs.clone(),
                structs: structs_handles
                    .into_iter()
                    .map(|h| h.join().unwrap())
                    .collect(),
                functions: functions_handles
                    .into_iter()
                    .map(|h| h.join().unwrap())
                    .collect(),
                modules: modules_handles
                    .into_iter()
                    .map(|h| h.join().unwrap())
                    .collect(),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: Id,
    pub path: Vec<String>,
    pub crate_id: u32,
    pub name: String,
    pub doc: Option<String>,
    pub visibility: Visibility,
    pub variants: Vec<Variant>,
}

impl Struct {
    pub fn parse(
        krate: &rustdoc_types::Crate,
        ztruct_item: &rustdoc_types::Item,
        ztruct: &rustdoc_types::Struct,
    ) -> Struct {
        let impls: Vec<Impl> = ztruct
            .impls
            .iter()
            .map(|imp_id| Impl::parse(krate, imp_id))
            .collect();

        // TODO parse variants

        let variants = vec![Variant { impls }];

        Struct {
            id: ztruct_item.id.clone(),
            path: krate.paths.get(&ztruct_item.id).unwrap().path.iter().cloned().collect(),
            crate_id: ztruct_item.crate_id,
            name: ztruct_item.name.clone().unwrap(),
            doc: ztruct_item.docs.clone(),
            visibility: ztruct_item.visibility.clone(),
            variants,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub impls: Vec<Impl>,
}


#[derive(Debug, Clone)]
pub struct Impl {
    pub id: Id,
    pub trait_: Option<Trait>,
    pub functions: Vec<Function>,
}

impl Impl {
    pub fn parse(krate: &rustdoc_types::Crate, imp_id: &Id) -> Self {
        let imp_item = krate.index.get(imp_id).unwrap();
        let imp = match &imp_item.inner {
            rustdoc_types::ItemEnum::Impl(imp) => imp,
            _ => unreachable!("Expected impl item"),
        };
        let functions = imp
            .items
            .iter()
            .filter_map(|id| {
                let item = krate.index.get(id).unwrap();
                match &item.inner {
                    rustdoc_types::ItemEnum::Function(f) => Some(Function::parse(krate, item, f)),
                    _ => None,
                }
            })
            .collect();
        
        let trait_ = if let Some(trait_) = imp.trait_.as_ref() {
            Some(Trait {
                id: trait_.id.clone(),
                path: krate.paths.get(&trait_.id).unwrap().path.iter().cloned().collect(),
            })
        } else {
            None
        };

        Self {
            id: imp_item.id.clone(),
            trait_: trait_,
            functions,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: Id,
    pub path: Vec<String>,
    pub name: String,
    pub doc: Option<String>,
    pub visibility: Visibility,
    pub func: rustdoc_types::Function,
}

impl Function {
    pub fn parse(krate: &rustdoc_types::Crate, item: &Item, func: &rustdoc_types::Function) -> Function {
        Function {
            id: item.id.clone(),
            doc: item.docs.clone(),
            visibility: item.visibility.clone(),
            path: krate.paths.get(&item.id).map(|x| x.path.iter().cloned().collect()).unwrap_or_default(),
            name: item.name.clone().unwrap(),
            func: func.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub id: Id,
    pub path: Vec<String>,
}
