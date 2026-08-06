pub fn code_span_metadata(
  mut input: Input
) -> IResult<Input, Content> {
  input.extra = "block_code_span";
}
