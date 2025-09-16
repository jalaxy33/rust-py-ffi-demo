"""
Basic functionality test for Python calling Rust functions
"""


def test_basic_functions():
    """Test basic functions"""
    try:
        import rust_py_ffi

        # Test sum_as_string function
        result_str = rust_py_ffi.sum_as_string(10, 20)
        print(f"sum_as_string(10, 20) = {result_str}")
        assert result_str == "30"

        # Test addition function
        result = rust_py_ffi.add(5, 3)
        print(f"add(5, 3) = {result}")
        assert result == 8

        # Test string length function
        length = rust_py_ffi.string_length("Hello, Rust!")
        print(f"string_length('Hello, Rust!') = {length}")
        assert length == 12

        # Test array processing function
        numbers = [1.0, 2.0, 3.0, 4.0, 5.0]
        processed = rust_py_ffi.process_numbers(numbers)
        print(f"process_numbers({numbers}) = {processed}")
        expected = [3.0, 5.0, 7.0, 9.0, 11.0]  # x * 2 + 1
        assert processed == expected

        # Test division function
        division_result = rust_py_ffi.divide(10.0, 2.0)
        print(f"divide(10.0, 2.0) = {division_result}")
        assert division_result == 5.0

        # Test division by zero error
        try:
            rust_py_ffi.divide(10.0, 0.0)
            assert False, "Should raise zero division error"
        except ZeroDivisionError as e:
            print(f"Zero division error handled correctly: {e}")

        print("✅ All basic function tests passed!")

    except ImportError as e:
        print(f"❌ Cannot import rust_py_ffi module: {e}")
        print("Please compile Rust module first: maturin develop")


def test_calculator_class():
    """Test calculator class"""
    try:
        import rust_py_ffi

        # Create calculator instance
        calc = rust_py_ffi.Calculator(10.0)
        print(f"Create calculator: {calc}")

        # Test addition
        result = calc.add(5.0)
        print(f"calc.add(5.0) = {result}")
        assert result == 15.0

        # Test multiplication
        result = calc.multiply(2.0)
        print(f"calc.multiply(2.0) = {result}")
        assert result == 30.0

        # Test property access
        print(f"calc.value = {calc.value}")
        assert calc.value == 30.0

        # Test reset
        calc.reset()
        print(f"After reset: {calc}")
        assert calc.value == 0.0

        print("✅ Calculator class tests passed!")

    except ImportError as e:
        print(f"❌ Cannot import rust_py_ffi module: {e}")
        print("Please compile Rust module first: maturin develop")


def main():
    """Main test function"""
    print("=" * 50)
    print("🧪 Basic Rust-Python FFI Tests")
    print("=" * 50)

    test_basic_functions()
    print()
    test_calculator_class()

    print("\n🎉 All basic tests completed!")


if __name__ == "__main__":
    main()
