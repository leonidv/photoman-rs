# Test

## Description of tests

### Arrange test
Tests check that all files processed correctly. It uses files in ```test_data/suite```. 
* The test checks that many files should be placed into one folder.
 
  Arranges images by date's folder. Files in `IMGP1011` have creation date 21 june 2020 year and thier should be placed into `2020-06-21` folder. 

*   The test checks that the folder, which name contains required date and has some text, is reused.

    The image `IMGP2011/03.JPG` should be placed into `2022-10-02 (Pushkin)`. 

* The test checks that conflict is processed by skipping the file.
 
  The image `IMGP2011/02.JPG` is ignored because the target folder `2022-10-02 Pushkin` already contains the file `02.JPG`.

### Dry-run test
The test checks that dry run mode does not made any changes of files.

## How to run test
`cargo test` - executes tests without application output (really, it's useless most times)
`cargo test -- --nocapture` - executes tests with application output

