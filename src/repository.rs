extern crate git2;

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub branch_type: git2::BranchType
}


/**
 * Returns branch list as Vec<Branch>
 */
pub fn branch_list(r: git2::Repository) -> Vec<Branch> {
    let mut branches: git2::Branches = r.branches(None).unwrap();
    let mut list: Vec<Branch> = Vec::new();

    while let Some(r) = branches.next() {
        let (branch, branch_type) = r.unwrap();

        // construct Branch model
        let b = Branch {
            name: branch.name().unwrap().unwrap().to_string(),
            branch_type: branch_type
        };

        list.push(b);
    }

    list
}


/**
 * Returns current branch name
 */
pub fn current_branch(r: git2::Repository) -> String {
    r.head().unwrap().shorthand().unwrap().to_string()
}