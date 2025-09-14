use serde::Serialize;


#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize)]
pub struct Heading {
    pub level: u32,
    pub id: String,
    pub permalink: String,
    pub title: String,
    pub children: Vec<Heading>,
}


impl Heading {
    pub fn new(level: u32) -> Heading  {
        Heading {
            level,
            // `Self` is a type that represents the current type being defined.
            // Here, `Self` is an alias for the `Heading` struct.
            // Using `Self` allows us to avoid having to write the struct name twice.

            ..Self::default()
        }
    }
}

/// Inserts a heading into its parent
/// 
/// # Arguments
/// * `possible_content` - A mutable reference to the parent heading
/// * `heading` - A reference to the heading to be inserted
/// 
/// # Returns
/// A boolean indicating whether the heading was inserted
fn insert_into_parent(possible_content: Option<&mut Heading>, heading: &Heading) -> bool {
    match possible_content  {
        Some(parent) => {
            if heading.level <=  parent.level{
                return false
            }

            if heading.level + 1 == parent.level + 1 {
                parent.children.push(heading.clone());
                return true; 
            }

            // avoid gooing deeper for now, this operation will have to be recursive operation

            true
        },
        None => { false }
    }
}


/// Makes a table of content from a list of headings
/// 
/// # Arguments
/// * `heading` - A vector of headings
/// 
/// # Returns
/// A vector of headings representing the table of content
pub fn make_table_of_content(heading : Vec<Heading>) -> Vec<Heading> {
    let mut toc = vec![]; 
    for heading in heading {
        if toc.is_empty() || !insert_into_parent(toc.iter_mut().last(), &heading) {
            toc.push(heading);
        }
    }

    toc
}