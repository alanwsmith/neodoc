pub fn code_span_metadata(
  mut input: Text
) -> IResult<Text, Span> {
  input.extra = "block_code_span";
}
