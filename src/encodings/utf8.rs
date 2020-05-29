// Generate invalid byte given a RNG
// Decode UTF-8-encoded BadSV:
// - Parse until invalid UTF-8
// - Split at delimiters, return vec of vec
// Encode UTF-8-flavored BadSV:
// - Get random invalid bytes
// - Write binary file, values joined with invalid delimiters