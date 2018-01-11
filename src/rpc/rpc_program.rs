/// Specification of a remote program.
///
/// This conforms to what's specified in the
/// [RPC Programs and Procedures][rfc_prog] of the RFC.
///
/// [rfc_prog]: https://tools.ietf.org/html/rfc1057#page-5
pub trait RpcProgram {
    /// Returns the program identification number.
    ///
    /// For more information, see the [Program Number Assignment][num_assign]
    /// section of the RFC.
    ///
    /// [num_assign]: https://tools.ietf.org/html/rfc1057#page-7
    fn program() -> u32;
    /// Returns the program version.
    fn version() -> u32;
}
