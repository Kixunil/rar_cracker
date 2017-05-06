extern crate unrar;

fn try_archive(file: &str, password: &str) -> bool {
    use unrar::archive::Archive;
    let archive = Archive::with_password(file.to_owned(), password.to_owned());
    let open_archive = archive.test()
                              .unwrap()
                              .process();
    open_archive.is_ok()
}

fn crack_rar<P: Into<String>, T: IntoIterator<Item=P>>(passwords: T) -> Option<String> {
    let mut cnt = 0;
    passwords.into_iter()
             .map(Into::into)
             .map(|password| {
                 if cnt > 100 {
                     println!("Trying: {}", password);
                     cnt = 0;
                 }
                 cnt += 1;
                 password
             })
             .find(|password| try_archive("Hello2.rar", &password))
}

struct BruteForceGen<'a> {
    state: Vec<usize>,
    alphabet: &'a [char],
}

impl<'a> BruteForceGen<'a> {
    fn new(alphabet: &'a [char], size: usize) -> Self {
        let mut state = Vec::with_capacity(size);
        for _ in 0..size {
            state.push(0)
        }

        BruteForceGen {
            state,
            alphabet,
        }
    }
}

impl<'a> Iterator for BruteForceGen<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        use std::mem::size_of;

        if self.state[0] == self.alphabet.len() {
            return None
        }

        let mut passwd = String::with_capacity(self.state.len()*size_of::<char>());
        for idx in &self.state {
            passwd.push(self.alphabet[*idx])
        }

        let mut last = true;
        for idx in &mut self.state {
            if *idx + 1 < self.alphabet.len() {
                *idx += 1;
                last = false;
                break;
            } else {
                *idx = 0
            }
        }
        if last {
            self.state[0] = self.alphabet.len();
        }

        Some(passwd)
    }
}

fn main() {
    //let password_list = vec!["Apple", "orange", "parketbar", "nbusr123", "progressbar", "raspberry"];
    let alphabet = ['h', 'e', 'l', 'o'];
    let password_list = BruteForceGen::new(&alphabet, 5);
    let result = crack_rar(password_list);
    match result {
        Some(password) => println!("Password is: {}", password),
        None => println!("Password not found"),
    }
}
