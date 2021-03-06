//
// Copyright (c) 2018, The MesaLock Linux Project Contributors
// All rights reserved.
//
// This work is licensed under the terms of the BSD 3-Clause License.
// For a copy, see the LICENSE file.
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//     Copyright (c) 2013-2018, Jordi Boggiano
//     Copyright (c) 2013-2018, Alex Lyon
//     Copyright (c) 2016       Jian Zeng
//
//     Permission is hereby granted, free of charge, to any person obtaining a
//     copy of this software and associated documentation files (the
//     "Software"), to deal in the Software without restriction, including
//     without limitation the rights to use, copy, modify, merge, publish,
//     distribute, sublicense, and/or sell copies of the Software, and to
//     permit persons to whom the Software is furnished to do so, subject to
//     the following conditions:
//
//     The above copyright notice and this permission notice shall be included
//     in all copies or substantial portions of the Software.
//
//     THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
//     OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
//     MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
//     IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//     CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
//     TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
//     SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

const NAME: &str = "base32";

#[test]
fn test_encode() {
    let input = "Hello, World!";
     new_cmd!()
        .with_stdin().buffer(input)
        .assert()
        .success()
        .stdout("JBSWY3DPFQQFO33SNRSCC===\n")
        .stderr("");
}

#[test]
fn test_decode() {
    for decode_param in vec!["-d", "--decode"] {
        let input = "JBSWY3DPFQQFO33SNRSCC===\n";
        new_cmd!()
            .arg(decode_param)
            .with_stdin().buffer(input)
            .assert()
            .success()
            .stdout("Hello, World!")
            .stderr("");
    }
}

#[test]
fn test_garbage() {
    let input = "aGVsbG8sIHdvcmxkIQ==\0";
    new_cmd!()
        .arg("-d")
        .with_stdin().buffer(input)
        .assert()
        .failure()
        .stdout("")
        .stderr(pred_str_contains!("invalid length at 16"));
}

#[test]
fn test_ignore_garbage() {
    for ignore_garbage_param in vec!["-i", "--ignore-garbage"] {
        let input = "JBSWY\x013DPFQ\x02QFO33SNRSCC===\n";
        new_cmd!()
            .args(&["-d", ignore_garbage_param])
            .with_stdin().buffer(input)
            .assert()
            .success()
            .stdout("Hello, World!")
            .stderr("");
    }
}

#[test]
fn test_wrap() {
    for wrap_param in vec!["-w", "--wrap"] {
        let input = "The quick brown fox jumps over the lazy dog.";
        new_cmd!()
            .args(&[wrap_param, "20"])
            .with_stdin().buffer(input)
            .assert()
            .success()
            .stdout("KRUGKIDROVUWG2ZAMJZG\n653OEBTG66BANJ2W24DT\nEBXXMZLSEB2GQZJANRQX\nU6JAMRXWOLQ=\n")
            .stderr("");
    }
}

#[test]
fn test_wrap_no_arg() {
    for wrap_param in vec!["-w", "--wrap"] {
        new_cmd!()
            .arg(wrap_param)
            .assert()
            .failure()
            .stdout("")
            .stderr(pred_str_contains!("requires a value but none was supplied\n"));
    }
}

#[test]
fn test_wrap_bad_arg() {
    for wrap_param in vec!["-w", "--wrap"] {
        new_cmd!()
            .args(&[wrap_param, "b"])
            .assert()
            .failure()
            .stdout("")
            .stderr(pred_str_contains!("'b' is not a number\n"));
    }
}

#[test]
fn test_encode_file() {
    new_cmd!()
        .current_dir(fixtures_dir!())
        .arg("plain.txt")
        .assert()
        .success()
        .stdout("JBSWY3DPFQQFO33SNRSCCCQ=\n")
        .stderr("");
}

#[test]
fn test_decode_file() {
    new_cmd!()
        .current_dir(fixtures_dir!())
        .args(&["-d", "encoded.txt"])
        .assert()
        .success()
        .stdout("The quick brown fox jumps over the lazy dog.\n")
        .stderr("");
}
