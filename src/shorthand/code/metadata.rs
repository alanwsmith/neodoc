pub fn code_span_metadata(
  mut input: Input
) -> IResult<Input, Span> {
  input.extra = "block_code_span";
}
