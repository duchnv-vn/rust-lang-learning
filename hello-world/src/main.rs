mod section_10_traits;
mod section_1_variables;
mod section_2_basic_data_types;
mod section_3_control_flows;
mod section_4_functions_closures;
mod section_5_stack_heap_ownership_borrowing;
mod section_6_standard_libs;
mod section_7_rust_data_types;
mod section_8_error_handling;
mod section_9_generic_type;

fn main() {
    println!("----- Hello Rust bootcamp 2024! -----");

    // SECTION 1: VARIABLES
    section_1_variables::main();

    // ---------------------------------------------------------
    // SECTION 2: DATA TYPES
    section_2_basic_data_types::main();

    // ---------------------------------------------------------
    // SECTION 3: CONTROL FLOW
    section_3_control_flows::main();

    // ---------------------------------------------------------
    // SECTION 4: FUNCTIONS & CLOSURES
    section_4_functions_closures::main();

    // ---------------------------------------------------------
    // SECTION 5: STACK/HEAP, OWNERSHIP/BORROWING
    section_5_stack_heap_ownership_borrowing::main();

    // ---------------------------------------------------------
    // SECTION 6: HASH MAP
    section_6_standard_libs::main();

    // ---------------------------------------------------------
    // SECTION 7: DATA TYPES
    section_7_rust_data_types::main();

    // ---------------------------------------------------------
    // SECTION 8: ERROR HANDLING
    section_8_error_handling::main();

    // ---------------------------------------------------------
    // SECTION 9: GENERIC TYPE
    section_9_generic_type::main();

    // ---------------------------------------------------------
    // SECTION 10: TRAITS
    section_10_traits::main();

    println!("----- End of main func! -----");
}
