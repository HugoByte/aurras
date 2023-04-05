# Workflow

## Testing
> The test accepts path to generated wasm as WORKFLOW_WASM env 
- Test workflow generated using ./examples/CarMarketPlace_Mock.yaml
```bash
cargo test test_car_market_place
```
- Test workflow generated using ./examples/EmployeeSalary_mock.yaml
```bash
cargo test test_employee_salary_with_concat_operator
```
- Test workflow generated using ./examples/Map_op_mock.yaml
```bash
cargo test test_map_operator
```