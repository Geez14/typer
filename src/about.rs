pub mod github {
    #[derive(Debug, PartialEq)]
    pub struct Profile<'a>(&'a str);

    impl Profile<'_> {
        pub fn github_link(&self) -> String {
            format!("https://github.com/{}", self.0)
        }
    }

    pub const GITHUB_ICON: char = '';
    pub const GITHUB_REPO_LINK: &str = "https://github.com/akash1047/typer";
    pub const CONTRIBUTERS: [Profile; 1] = [
        Profile("Geez14")
        Profile("akash1047"),
    ];
}
