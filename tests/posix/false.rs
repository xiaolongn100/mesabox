//
// Copyright (c) 2018, The MesaLock Linux Project Contributors
// All rights reserved.
// 
// This work is licensed under the terms of the BSD 3-Clause License.
// For a copy, see the LICENSE file.
//

use util::*;

const NAME: &str = "false";

#[test]
fn test_false() {
    new_ucmd!().fails();
}