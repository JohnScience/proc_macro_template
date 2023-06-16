import sys

macro_core_crate = '{{ cookiecutter.__macro_core_crate }}'

def is_valid_crate_name(crate_name):
    is_properly_delimited = not ("-" in crate_name and "_" in crate_name)
    return is_properly_delimited and crate_name.islower()

def is_available_on_crates_io(crate_name):
    # TODO: check if the crate name is available on crates.io
    # One way to do this is to write a Python module with Rust bindings.
    # There's a `checker` crate that does this:
    # https://crates.io/crates/checker
    return True

if __name__ == '__main__':
    # exits with status 1 indicate failure

    if not is_valid_crate_name(macro_core_crate):
        print('ERROR: %s is not a valid crate name!' % macro_core_crate)
        sys.exit(1)
    if not is_available_on_crates_io(macro_core_crate):
        print('ERROR: %s is not available on crates.io!' % macro_core_crate)
        sys.exit(1)
