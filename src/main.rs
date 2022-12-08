extern crate core;

use std::env::args;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, Read};
use csv::Reader;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use crate::Error::{MissingMatch, NumberParseError};

lazy_static!{
    static ref LINE_PATTERN: Regex = Regex::new(r"(?:\S+\s+){6}(\S+)\s+(\S+)\s+(\S+).+").expect("Invalid regex");
}

#[derive(Debug)]
enum Error {
    NumberParseError(String),
    LineReadFailed,
    LineMatchFailed(String),
    MissingMatch
}

fn main() {
    let mut file = match args().skip(1).next() {
        None => {
            eprintln!("Provide a file as an argument");
            return
        },
        Some(path) => {File::open(path)}
    };

    match &mut file {
        Ok(f) => match calculate(f){
            Ok(result) => {
                println!("{}", result);
            }
            Err(e) => eprintln!("Failed to calculate value {:?}", e)
        }
        Err(e) => eprintln!("Failed to open file {}", e)
    }
    io::stdin().read(&mut [0; 1]);
}

fn calculate(file: &mut File) -> Result<f64, Error> {


    let reader = io::BufReader::new(file);
    let lines = reader.lines().skip(1).map(|l|l.map_err(|_|Error::LineReadFailed)).fold(Ok(Vec::new()), |a, b|{
        match a {
            Err(_) => a,
            Ok(mut x) => match b {
                Err(e) => Err(e),
                Ok(y) => {x.push(y); Ok(x)}
            }
        }
    })?;
    println!("Molecules: {}", lines.len());
    let parsed_lines = lines.iter().map(|l|{(l, LINE_PATTERN.captures(l))}).fold(Ok(Vec::new()), |mut v, (l, n)|{
        match v {
            Err(_) => v,
            Ok(mut r) => match n {
                None => Err(Error::LineMatchFailed(l.to_string())),
                Some(l) => {
                    let line_match = l.iter().skip(1).fold(Ok(Vec::new()), |mut l, n|{
                        match l {
                            Err(_) => l,
                            Ok(mut v) => match n {
                                None => Err(MissingMatch),
                                Some(m) => {v.push(m.as_str().to_string()); Ok(v)}
                            }
                        }
                    });
                    match line_match {
                        Ok(l) => {r.push(l);Ok(r)}
                        Err(e) => Err(e)
                    }
                }
            }
        }
    })?;
    let values = parsed_lines.iter().map(|s|{
        s.iter()
            .map(|l|l.parse::<f64>()
                .map_err(|_|NumberParseError(l.to_string())))
            .fold(Ok(Vec::new()), |a, b|{
            match a {
                Err(_) => a,
                Ok(mut x) => match b {
                    Ok(y) => {x.push(y); Ok(x)},
                    Err(e) => Err(e)
                }
            }
        })
    }).fold(Ok(Vec::new()),
            |mut a, b| {
                match a {
                    Err(_) => a,
                    Ok(mut x) => match b {
                        Ok(y) => {x.push(y); Ok(x)},
                        Err(e) => Err(e)
                    }
                }
            }
    )?;
    println!("Input:");
    values.iter().for_each(|v|println!("{:?}", v));
    println!("Input squared:");
    let rows_squared = values.iter().map(|v|v.iter().map(|f|f*f).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>();
    rows_squared.iter().for_each(|v|println!("{:?}", v));
    println!("summed:");
    let sums = rows_squared.iter().map(|v|v.iter().sum()).collect::<Vec<f64>>();
    sums.iter().for_each(|v|println!("{}", v));
    println!("sqrt:");
    let sqrts = sums.iter().map(|v|v.sqrt()).collect::<Vec<f64>>();
    sqrts.iter().for_each(|v|println!("{}", v));
    println!("sum:");
    let sqrts_sum:f64 = sqrts.iter().sum();
    println!("{}", sqrts_sum);
    println!("avg:");
    let sqrts_avg: f64 = sqrts_sum / sqrts.len() as f64;
    println!("{}", sqrts_avg);
    println!("sqrt result");
    let result = sqrts_avg.sqrt();
    println!("{}", result);

    let calculation: f64 = (values.iter().map(
        |r|r.iter().map(|v|v.powf(2.0)).sum::<f64>().sqrt()
    ).sum::<f64>() / values.len() as f64).sqrt();
    println!("n={}", values.len());
    assert!(result == calculation);
    Ok(result)
}
