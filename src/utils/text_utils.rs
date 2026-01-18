use slug::slugify;

pub fn to_slug(text: &str) -> String {
    slugify(text)
}