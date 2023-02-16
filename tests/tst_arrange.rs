
extern crate chrono;
use std::{fs};

use photoman::Manager;
use spectral::assert_that;
use spectral::prelude::PathAssertions;

mod prepare_suite;

use prepare_suite::prepare_suite;

#[test]
pub fn standard_execution() {
    let r = prepare_suite("standard_execution");

    if let Ok(test_dir) = r {
        let mut manager = Manager::new().work_dir(&test_dir);
        manager.arrange_files();

        let dir_2020_06_21 = test_dir.join("2020-06-21");
        let dir_2020_06_21_raw = dir_2020_06_21.join("raw");
        let dir_2022_10_02 = test_dir.join("2022-10-02 (Pushkin)");        

        assert_that(&dir_2020_06_21).exists();
        assert_that(&dir_2020_06_21.join("01.jpg")).exists();
        assert_that(&dir_2020_06_21_raw).exists();

        assert_that(&dir_2022_10_02.join("02.JPG")).exists();
        assert_that(&dir_2022_10_02.join("03.JPG")).exists();
        
        assert_that(&test_dir.join("2022-10-02")).does_not_exist();

        assert_that(&test_dir.join("IMGP2011").join("02.JPG")).exists();
        assert_that(&test_dir.join("IMGP1011")).does_not_exist();

        fs::remove_dir_all(&test_dir).unwrap();
    }

}

#[allow(non_snake_case)]
#[test]
pub fn dry_run() {
    let r = prepare_suite("dry_run");

    if let Ok(test_dir) = r {
        let mut manager = Manager::new()
            .work_dir(&test_dir)
            .dry_run();

        manager.arrange_files();

        let dir_2020_06_21 = test_dir.join("2020-06-21");
        let dir_2022_10_02 = test_dir.join("2022-10-02 (Pushkin)");        

        let dir_IMGP1011 = test_dir.join("IMGP1011");
        let dir_IMGP2011 = test_dir.join("IMGP2011");

        assert_that(&dir_2020_06_21).does_not_exist();

        assert_that(&dir_IMGP1011).exists();
        assert_that(&dir_IMGP1011.join("01.jpg")).exists();
        assert_that(&dir_IMGP1011.join("01.raw")).exists();

        assert_that(&dir_IMGP2011.join("02.JPG")).exists();
        assert_that(&dir_IMGP2011.join("03.JPG")).exists();
        
        assert_that(&dir_2022_10_02.join("02.JPG")).exists();

        assert_that(&test_dir.join("2022-10-02")).does_not_exist();

        fs::remove_dir_all(&test_dir).unwrap();
    }

}