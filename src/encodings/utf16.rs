// Generate invalid byte given a RNG
// Decode UTF-16-encoded BadSV:
// - Parse until invalid UTF-16
// - Split at delimiters, return vec of vec
// Encode UTF-16-flavored BadSV:
// - Get random invalid bytes
// - Write binary file, values joined with invalid delimiters