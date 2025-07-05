use std::{io, vec};
use rand::Rng;

fn main() {

    let mut x = 0;
    let mut y = 0;

    let mut room = 0;

    let mut smer = String::from("nahoru");

    let mut mapa: Vec<Vec<i32>> = Vec::new();
    mapa.push(vec![0]);

    let mut rng = rand::rng();

    let dej = ["Jsi v chodbě.", "Před tebou je zeď. ", "Před tebou je nepřítel.", "Jsi na křižovatce."];

    println!("Vítej dobrodruhu!");

    loop {
        let mut cislo = rng.random_range(0..dej.len() as i32);
        println!("{}", smer);

        vyber_mistnosti(&mut mapa, &mut smer, &mut cislo, y, x);
        
        if cislo == 0 {
            chodba(&dej, &mut x, &mut y, &mut smer, &mut mapa, room);
        } else if cislo == 3 {
            krizovatka(&dej, &mut x, &mut y, &mut smer, &mut mapa, room);
        } else if cislo == 1 {
            zed(&dej, &mut x, &mut y, &mut smer, &mut mapa, room);
        }
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
fn chodba(list: &[&str], mut x: &mut i32, mut y: &mut i32,smer: &mut String, mapa: &mut Vec<Vec<i32>>, mut room: i32) {
    println!("{}",list[0]);
    loop {
        for i in &mut *mapa {
            println!(" ");
            for j in i {
                print!("{} ", j);
            }
        }
        println!(" ");
        println!("x: {}", x);
        println!("y: {}", y);
        let odpoved = txt_vstup("Půjdeš rovně, nebo zpět?").trim().to_string();
        if odpoved == "rovně" {
            println!("Jdeš rovně.");

            prace_s_vektorem(list, &mut x, &mut y, smer, mapa, room, 1);
            return;

        } else if odpoved == "zpět" {
            println!("Otočil ses a jdeš zpátky");
            
            prace_s_vektorem(list, &mut x, &mut y, smer, mapa, room, 1);
            opacny_smer(smer);
            return;

        } else {
            println!("Promiň, nerozuměl jsem")
        }
    }
}

fn opacny_smer(smer: &mut String) -> String {
    if smer == "nahoru" {
        *smer = "dolů".to_string();
    } else if smer == "dolů" {
        *smer = "nahoru".to_string();
    } else if smer == "doleva" {
        *smer = "doprava".to_string();
    } else if smer == "doprava" {
        *smer = "doleva".to_string();
    }
    return smer.to_string();
}

fn krizovatka(list: &[&str], mut x: &mut i32, mut y: &mut i32,smer: &mut String, mapa: &mut Vec<Vec<i32>>, mut room: i32) {
    println!("{}",list[3]);
    loop {
        for i in &mut *mapa {
            println!(" ");
            for j in i {
                print!("{} ", j);
            }
        }
        println!(" ");
        println!("x: {}", x);
        println!("y: {}", y);
        let mut odpoved = txt_vstup("Kterým směrem půjdeš?").trim().to_string();
        if odpoved == "rovně" {
            println!("Jdeš rovně.");
            
            prace_s_vektorem(list, &mut x, &mut y, smer, mapa, room, 4);
            return;

        } else if odpoved == "zpět" {
            println!("Otočil ses a jdeš zpátky");

            prace_s_vektorem(list, &mut x, &mut y, smer, mapa, room, 4);
            opacny_smer(smer);
            return;

        } else if odpoved == "vpravo" {
            println!("Vykročil jsi vpravo");

            prace_s_vektorem(list, &mut x, &mut y, smer, mapa, room, 4);
            zatacka(smer, &mut odpoved);
            return;

        } else if odpoved == "vlevo" {
            println!("Vykročil jsi vlevo");

            prace_s_vektorem(list, &mut x, &mut y, smer, mapa, room, 4);
            zatacka(smer, &mut odpoved);
            return;

        } else {
            println!("Promiň, nerozuměl jsem")
        }
    }
}

fn zed(list: &[&str], mut x: &mut i32, mut y: &mut i32,smer: &mut String, mapa: &mut Vec<Vec<i32>>, mut room: i32) {
    println!("{}",list[1]);
    loop {
        for i in &mut *mapa {
            println!(" ");
            for j in i {
                print!("{} ", j);
            }
        }
        println!(" ");
        println!("x: {}", x);
        println!("y: {}", y);
        let odpoved = txt_vstup("Půjdeš zpět?").trim().to_string();
        if odpoved == "zpět" {
            println!("Jdeš zpět.");

            prace_s_vektorem(list, &mut x, &mut y, smer, mapa, room, 2);
            opacny_smer(smer);
            return;

        } else {
            println!("Promiň, nerozuměl jsem.");
        }
    }
}

fn zatacka(smer: &mut String, odpoved: &mut String) {
    if *smer == "nahoru" {
        if *odpoved == "vpravo" {
            *smer = "doprava".to_string();          
        } else if *odpoved == "vlevo" {   
            *smer = "doleva".to_string();          
        } else if *odpoved == "rovně" {
            *smer = "nahoru".to_string();           
        } else if *odpoved == "zpět" {
            *smer = "dolů".to_string();          
        }
    } else if *smer == "doprava" {
        if *odpoved == "vpravo" {
            *smer = "dolů".to_string();           
        } else if *odpoved == "vlevo" {   
            *smer = "nahoru".to_string();           
        } else if *odpoved == "rovně" {
            *smer = "doprava".to_string();           
        } else if *odpoved == "zpět" {
            *smer = "doleva".to_string();          
        }
    } else if *smer == "dolů" {
        if *odpoved == "vpravo" {
            *smer = "doleva".to_string();           
        } else if *odpoved == "vlevo" {   
            *smer = "doprava".to_string();           
        } else if *odpoved == "rovně" {
            *smer = "dolů".to_string();           
        } else if *odpoved == "zpět" {
            *smer = "nahoru".to_string();           
        }
    } else if *smer == "doleva" {
        if *odpoved == "vpravo" {
            *smer = "nahoru".to_string();           
        } else if *odpoved == "vlevo" {   
            *smer = "dolů".to_string();           
        } else if *odpoved == "rovně" {
            *smer = "doleva".to_string();            
        } else if *odpoved == "zpět" {
            *smer = "doprava".to_string();
        }
    }
}

fn prace_s_vektorem(list: &[&str], mut x: &mut i32, mut y: &mut i32,smer: &mut String, mapa: &mut Vec<Vec<i32>>, mut room: i32, cislo_ve_vektoru: i32) {
    if smer == "nahoru" {
        if *y == 0 {
            let vektor = vec![0; (mapa[0].len()) as usize];
            mapa.insert(0, vektor);
            mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
        } else if *y > 0 {
            *y -= 1;
            if mapa[*y as usize][*x as usize] == 0 {
                mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
            }
        }
    } else if smer == "dolů" {
        if *y == mapa.len() as i32 - 1 {
            let vektor = vec![0; (mapa[0].len()) as usize];
            *y += 1;
            mapa.insert(*y as usize, vektor);
            mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
            if mapa[*y as usize][*x as usize] == 0 {
                *y += 1;
                mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
            }
        } else if *y != mapa.len() as i32 - 1 {
            if mapa[(*y + 1) as usize][*x as usize] != 0 {
                *y += 1;
                room = mapa[*y as usize][*x as usize];
            } else if mapa[(*y + 1) as usize][*x as usize] == 0 {
                *y += 1;
                mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
            }
        }
    } else if smer == "doleva" {
        if *x == 0 {
            for vektor in &mut *mapa {
                vektor.insert(0, 0);
            }
            mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
        } else if *x > 0 {
            if mapa[*y as usize][(*x - 1) as usize] != 0 {
                *x -= 1;
                room = mapa[*y as usize][*x as usize];
            } else if mapa[*y as usize][(*x - 1) as usize] == 0 {
            *x -= 1;
            mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
            }
        }
    } else if smer == "doprava" {
        if *x == mapa[0].len() as i32 - 1 {
            for vektor in &mut *mapa {
                vektor.push(0);
            }
            *x += 1;
            mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
        } else if *x != mapa[0].len() as i32 - 1 {
                *x += 1;
            if mapa[*y as usize][*x as usize] == 0 {
                mapa[*y as usize][*x as usize] = cislo_ve_vektoru;
            } else {
                room = mapa[*y as usize][*x as usize];
            }
        }
    }  
}

fn vyber_mistnosti(mapa: &mut Vec<Vec<i32>>, smer: &str, cislo: &mut i32, y: i32, x: i32) -> i32 {
    if smer == "nahoru" && y > 0 {
        let hodnota = mapa[(y - 1) as usize][x as usize];
        if hodnota != 0 {
            *cislo = hodnota - 1;
        }
    }

    if smer == "dolů" && y < (mapa.len() - 1) as i32 {
        let hodnota = mapa[(y + 1) as usize][x as usize];
        if hodnota != 0 {
            *cislo = hodnota - 1;
        }
    }

    if smer == "doleva" && x > 0 {
        let hodnota = mapa[y as usize][(x - 1) as usize];
        if hodnota != 0 {
            *cislo = hodnota - 1;
        }
    }

    if smer == "doprava" && x < (mapa[0].len() - 1) as i32 {
        let hodnota = mapa[y as usize][(x + 1) as usize];
        if hodnota != 0 {
            *cislo = hodnota - 1;
        }
    }

    *cislo
}
