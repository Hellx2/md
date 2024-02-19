use std::fs;

// TODO: Add support for maths
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut ao = false;
    let mut to = false;
    let mut title = String::new();
    let mut author = String::new();
    for i in args[1..].iter() {
        if ao {
            author = i.clone();
            ao = false;
        } else if to {
            title = i.clone();
            to = false;
        } else if i.starts_with('-') {
            if i.starts_with("-a") {
                if i.strip_prefix("-a").unwrap().starts_with('=') {
                    author = i[3..].to_string();
                } else {
                    ao = true;
                }
            } else if i.starts_with("--author") {
                if i.strip_prefix("--author").unwrap().starts_with('=') {
                    author = i[9..].to_string();
                } else {
                    ao = true;
                }
            } else if i.starts_with("-t") {
                if i.strip_prefix("-t").unwrap().starts_with('=') {
                    title = i[3..].to_string();
                } else {
                    to = true;
                }
            } else if i.starts_with("--title") {
                if i.strip_prefix("--title").unwrap().starts_with('=') {
                    title = i[8..].to_string();
                } else {
                    to = true;
                }
            } 
        } else if fs::metadata(i).is_ok() {
            let mut rval = String::new();
            let f = fs::read_to_string(i).unwrap();
            let (mut listopen, mut listordered, mut tc, mut listitem, mut linenum, mut tr, mut it, mut ith) = (false, false, 3, 1, 1, 0, false, false);
            add_line(&mut rval, "<html lang='en'>", 1);
            add_line(&mut rval, "<head>", 2);
            add_lines(&mut rval, vec!["<meta charset='utf-8'>", format!("<meta name='author' content=\"{}\">", author.as_str()).as_str(), format!("<title>{}</title>", title.as_str()).as_str()], 3);
            add_lines(&mut rval, vec!["</head>", "<body>"], 2);
            for line in f.lines() { 
                if !line.trim().starts_with(format!("{listitem}.").as_str()) && listopen && listordered {
                    listitem = 1;
                    listopen = false;
                    listordered = false;
                    tc -= 1;
                    add_line(&mut rval, "</ol>", tc);
                }
                if !(line.trim().starts_with('-') || line.trim().starts_with("--") || line.trim().starts_with(format!("{listitem}.").as_str())) && listopen {
                    listopen = false;
                    tc -= 1;
                    add_line(&mut rval, if listordered {"</ol>\n"} else {"</ul>\n"}, tc);
                    listordered = false;
                }
                if it && !line.trim().starts_with('|') {
                    it = false;
                    tc -= 1;
                    add_line(&mut rval, "</table>", tc); 
                }
                if line.trim().starts_with('-') && !line.trim().starts_with("--") {
                    if !listopen || listordered {
                        listopen = true;
                        if listordered {
                            tc -= 1;
                            add_line(&mut rval, "</ol>", tc);
                        } else {
                            add_line(&mut rval, "</ul>", tc);
                        }
                        add_line(&mut rval, "<ul>", tc);
                        tc += 1;
                    }
                    add_line(&mut rval, format!("<li>{}</li>", line.trim()[1..].trim()).as_str(), tc);
                } else if line.trim().starts_with(format!("{listitem}.").as_str()) {
                    if !listopen || !listordered {
                        if listopen {
                            tc -= 1;
                            add_line(&mut rval, "</ul>", tc);
                        }
                        listopen = true;
                        add_line(&mut rval, "<ol>", tc);
                        tc += 1;
                    }
                    add_line(&mut rval, format!("<li>{}</li>", line.trim()[(if listordered {2} else {1})..].trim()).as_str(), tc);
                    listordered = true;
                    listitem += 1;
                } else if line.trim().starts_with('#') {
                    let mut j = 1;
                    for i in 1..6 {
                        if !line.trim()[i..].starts_with('#') {
                            break;
                        }
                        j += 1;
                    }
                    add_line(&mut rval, format!("<h{j}>{}</h{j}>", line.trim()[j..].trim()).as_str(), tc);
                } else if line.trim().starts_with('|') {
                    if !line.trim().ends_with('|') {
                        eprintln!("\u{001b}[31;1mError:\u{001b}[0m Line of table does not end with '|', line {}", linenum);
                        std::process::exit(1);
                    }
                    if !it && !ith {
                        ith = true;
                        add_line(&mut rval, "<table>", tc);
                        tc += 1;
                        add_line(&mut rval, "<th>", tc);
                        tc += 1;
                        for i in line[1..].split('|').rev().collect::<Vec<&str>>().iter().skip(1).rev() {
                            for _ in 1..tc {
                                rval.push_str("    ");
                            }
                            rval.push_str("<td>");
                            rval.push_str(i.trim());
                            rval.push_str("</td>\n");
                            tr += 1;
                        }
                        tc -= 1;
                        for _ in 1..tc {
                            rval.push_str("    ");
                        }
                        rval.push_str("</th>\n");
                    } else if !it {
                        ith = false;
                        it = true;
                        if line[1..].split('|').rev().collect::<Vec<&str>>().iter().skip(1).rev().count() != tr {
                            eprintln!("\u{001b}[31;1mError:\u{001b}[0m Not enough columns in table, line {}", linenum);
                            std::process::exit(1);
                        } 
                    } else {
                        for _ in 1..tc {
                            rval.push_str("    ");
                        }
                        rval.push_str("<tr>\n");
                        tc += 1;
                        for i in line[1..].split('|').rev().collect::<Vec<&str>>().iter().skip(1).rev() {
                            for _ in 1..tc {
                                rval.push_str("    ");
                            }
                            rval.push_str("<td>");
                            rval.push_str(i.trim());
                            rval.push_str("</td>\n");
                            tr += 1;
                        }
                        tc -= 1;
                        for _ in 1..tc {
                            rval.push_str("    ");
                        }
                        rval.push_str("</tr>\n");
                    }
                } else {
                    rval.push_str(line.trim());
                    rval.push('\n');
                }
                linenum += 1;
            }
            if it {
                tc -= 1;
                for _ in 1..tc {
                    rval.push_str("    ");
                }
                rval.push_str("</table>\n");
            }
            if listopen {
                tc -= 1;
                add_line(&mut rval, if listordered {"</ol>\n"} else {"</ul>\n"}, tc);
            }
            rval.push_str("    </body>\n</html>");
            println!("{}", rval);
        } else {
            eprintln!("\u{001b}[31;1mError:\u{001b}[0m Failed to read file {}.", i);
            std::process::exit(1);
        }
    }
}

fn add_line(rval: &mut String, line: &str, tabn: i32) {
    for _ in 1..tabn {
        rval.push_str("    ");
    }
    rval.push_str(line);
    rval.push('\n');
}

fn add_lines(rval: &mut String, lines: Vec<&str>, tabn: i32) {
    for line in lines {
        add_line(rval, line, tabn);
    }
}
