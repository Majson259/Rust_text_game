use std::{io, vec};
use rand::Rng;

fn main() {

    let mut x = 0;
    let mut y = 0;

    let mut room = 0;

    let mut smer = "doprava";

    let mut mapa: Vec<Vec<i32>> = Vec::new();
    mapa.push(vec![0]);

    let mut rng = rand::thread_rng();

    let dej = ["Jsi v chodbě.", "Před tebou něco je, ", "Před tebou je nepřítel.", "Jsi na křižovatce."];

    println!("Vítej dobrodruhu!");

    loop {
        let cislo = rng.random_range(0..dej.len());
        
        chodba(&dej, x, y, smer, &mut mapa, room);
    }

}

fn txt_vstup(prompt: &str) -> String {

    loop {
    println!("{}", prompt);
    let mut vstup = String::new();
    io::stdin().read_line(&mut vstup).expect("Chyba při čtení");
    return vstup;
    }
}

fn zpracuj_vstup(text: &str) -> String {
    text.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect()
}

// Místnosti
fn chodba(list: &[&str], mut x: i32, mut y: i32, smer: &str, mapa: &mut Vec<Vec<i32>>, mut room: i32) {
    println!("{}",list[0]);
    loop {
        let odpoved = txt_vstup("Půjdeš rovně, nebo zpět?").trim().to_string();
        if odpoved == "rovně" {
            println!("Jdeš rovně.");
            if smer == "nahoru" {
                if y == 0 {
                    let vektor = vec![0; (x + 1) as usize];
                    mapa.insert(0, vektor);
                    mapa[y as usize][x as usize] = 1;
                    return;
                } else if y > 0 {
                    y -= 1;
                    room = mapa[y as usize][x as usize];
                    return;
                }
            } else if smer == "dolů" {
                if y == mapa.len() as i32 {
                    let vektor = vec![0; x as usize];
                    mapa.insert(y as usize, vektor);
                    return;
                }
            } else if smer == "doleva" {
                if x == 0 {
                    for vektor in &mut *mapa {
                        vektor.insert(0, 0);
                    }
                    mapa[y as usize][x as usize] = 1;
                    return;
                } else if x > 0 {
                    if mapa[y as usize][(x - 1) as usize] != 0 {
                        x -= 1;
                        room = mapa[y as usize][x as usize];
                        return;
                    }
                }
            } else if smer == "doprava" {
                if x == mapa[0].len() as i32 {
                    for vektor in &mut *mapa {
                        vektor.push(0);
                    }
                    x += 1;
                    mapa[y as usize][x as usize] = 1;
                    return;
                } else if x != mapa[0].len() as i32 - 1 {
                    if mapa[y as usize][(x + 1) as usize] != 2 {
                    x += 1;
                    }
                    room = mapa[y as usize][x as usize];
                    return;
                }
            }

        } else if odpoved == "zpět" {
            println!("Otočil ses a jdeš zpátky");
            return;
        } else {
            println!("Promiň, nerozuměl jsem")
        }
    }
}

fn krizovatka(list: &[i32], x: i32, y: i32) {
    println!("{}",list[3]);
    loop {
        let odpoved = txt_vstup("Kterým směrem půjdeš?");
        if odpoved == "rovně" {
            println!("Jdeš rovně.");
            return;
        } else if odpoved == "zpět" {
            println!("Otočil ses a jdeš zpátky");
            return;
        } else if odpoved == "vpravo" {
            println!("Vykročil jsi vpravo");
            return;
        } else if odpoved == "vlevo" {
            println!("Vykročil jsi vlevo");
            return;
        }
         else {
            println!("Promiň, nerozuměl jsem")
        }
    }
}
