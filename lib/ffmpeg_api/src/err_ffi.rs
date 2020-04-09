use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum AvFfiError {
    #[error("Operation not permitted")]
    OperationNotPermitted = 1,
    #[error("No such file or directory")]
    NoSuchFileOrDirectory = 2,
    #[error("No such process")]
    NoSuchProcess = 3,
    #[error("Interrupted system call")]
    InterruptedSystemCall = 4,
    #[error("I/O error")]
    IoError = 5,
    #[error("No such device or address")]
    NoSuchDeviceOrAddress = 6,
    #[error("Argument list too long")]
    ArgumentListTooLong = 7,
    #[error("Exec format error")]
    ExecFormatError = 8,
    #[error("Bad file number")]
    BadFileNumber = 9,
    #[error("No child processes")]
    NoChildProcesses = 10,
    #[error("Try again")]
    TryAgain = 11,
    #[error("Out of memory")]
    OutOfMemory = 12,
    #[error("Permission denied")]
    PermissionDenied = 13,
    #[error("Bad address")]
    BadAddress = 14,
    #[error("Block device required")]
    BlockDeviceRequired = 15,
    #[error("Device or resource busy")]
    DeviceOrResourceBusy = 16,
    #[error("File exists")]
    FileExists = 17,
    #[error("Cross-device link")]
    CrossDeviceLink = 18,
    #[error("No such device")]
    NoSuchDevice = 19,
    #[error("Not a directory")]
    NotDirectory = 20,
    #[error("Is a directory")]
    Directory = 21,
    #[error("Invalid argument")]
    InvalidArgument = 22,
    #[error("File table overflow")]
    FileTableOverflow = 23,
    #[error("Too many open files")]
    TooManyOpenFiles = 24,
    #[error("Not a typewriter")]
    NotATypewriter = 25,
    #[error("Text file busy")]
    TextFileBusy = 26,
    #[error("File too large")]
    FileTooLarge = 27,
    #[error("No space left on device")]
    NoSpaceLeftOnDevice = 28,
    #[error("Illegal seek")]
    IllegalSeek = 29,
    #[error("Read-only file system")]
    ReadOnlyFileSystem = 30,
    #[error("Too many links")]
    TooManyLinks = 31,
    #[error("Broken pipe")]
    BrokenPipe = 32,
    #[error("Math argument out of domain of func")]
    MathArgumentOutOfDomain = 33,
    #[error("Math result not representable")]
    MathResultNotRepresentable = 34,
    #[error("Resource deadlock would occur")]
    ResourceDeadlock = 35,
    #[error("File name too long")]
    FileNameTooLong = 36,
    #[error("No record locks available")]
    NoRecordLocksAvailable = 37,
    #[error("Invalid system call number")]
    InvalidSystemCall = 38,
    #[error("Directory not empty")]
    DirectoryNotEmpty = 39,
    #[error("Too many symbolic links encountered")]
    TooManySymbolicLinks = 40,
    #[error("No message of desired type")]
    NoMessageOfDesiredType = 42,
    #[error("Identifier removed")]
    IdentifierRemoved = 43,
    #[error("Channel number out of range")]
    ChannelNumberOutOfRange = 44,
    #[error("Level 2 not synchronized")]
    Level2NotSynchronized = 45,
    #[error("Level 3 halted")]
    Level3Halted = 46,
    #[error("Level 3 reset")]
    Level3Reset = 47,
    #[error("Link number out of range")]
    LinkNumberOutOfRange = 48,
    #[error("Protocol driver not attached")]
    ProtocolDriverNotAttached = 49,
    #[error("No CSI structure available")]
    NoCsiStructureAvailable = 50,
    #[error("Level 2 halted")]
    Level2Halted = 51,
    #[error("Invalid exchange")]
    InvalidExchange = 52,
    #[error("Invalid request descriptor")]
    InvalidRequestDescriptor = 53,
    #[error("Exchange full")]
    ExchangeFull = 54,
    #[error("No anode")]
    NoAnode = 55,
    #[error("Invalid request code")]
    InvalidRequestCode = 56,
    #[error("Invalid slot")]
    InvalidSlot = 57,
    #[error("Bad font file format")]
    BadFontFileFormat = 59,
    #[error("Device not a stream")]
    DeviceNotAStream = 60,
    #[error("No data available")]
    NoDataAvailable = 61,
    #[error("Timer expired")]
    TimerExpired = 62,
    #[error("Out of streams resources")]
    OutOfStreamsResources = 63,
    #[error("Machine is not on the network")]
    MachineNotOnNetwork = 64,
    #[error("Package not installed")]
    PackageNotInstalled = 65,
    #[error("Object is remote")]
    ObjectRemote = 66,
    #[error("Link has been severed")]
    LinkSevered = 67,
    #[error("Advertise error")]
    AdvertiseError = 68,
    #[error("Srmount error")]
    SrmountError = 69,
    #[error("Communication error on send")]
    CommunicationErrorOnSend = 70,
    #[error("Protocol error")]
    ProtocolError = 71,
    #[error("Multihop attempted")]
    MultihopAttempted = 72,
    #[error("RFS specific error")]
    RfsSpecificError = 73,
    #[error("Not a data message")]
    NotADataMessage = 74,
    #[error("Value too large for defined data type")]
    ValueTooLarge = 75,
    #[error("Name not unique on network")]
    NameNotUniqueOnNetwork = 76,
    #[error("File descriptor in bad state")]
    FileDescriptorInBadState = 77,
    #[error("Remote address changed")]
    RemoteAddressChanged = 78,
    #[error("Can not access a needed shared library")]
    CanNotAccessSharedLibrary = 79,
    #[error("Accessing a corrupted shared library")]
    CorruptedSharedLibrary = 80,
    #[error(".lib section in a.out corrupted")]
    LibSectionCorrupted = 81,
    #[error("Attempting to link in too many shared libraries")]
    TooManySharedLibraries = 82,
    #[error("Cannot exec a shared library directly")]
    CannotExecASharedLibraryDirectly = 83,
    #[error("Illegal byte sequence")]
    IllegalByteSequence = 84,
    #[error("Interrupted system call should be restarted")]
    RestartInterruptedSystemCall = 85,
    #[error("Streams pipe error")]
    StreamsPipeError = 86,
    #[error("Too many users")]
    TooManyUsers = 87,
    #[error("Socket operation on non-socket")]
    SocketOperationOnNonSocket = 88,
    #[error("Destination address required")]
    DestinationAddressRequired = 89,
    #[error("Message too long")]
    MessageTooLong = 90,
    #[error("Protocol wrong type for socket")]
    ProtocolWrongTypeForSocket = 91,
    #[error("Protocol not available")]
    ProtocolNotAvailable = 92,
    #[error("Protocol not supported")]
    ProtocolNotSupported = 93,
    #[error("Socket type not supported")]
    SocketTypeNotSupported = 94,
    #[error("Operation not supported on transport endpoint")]
    OperationNotSupportedOnTransportEndpoint = 95,
    #[error("Protocol family not supported")]
    ProtocolFamilyNotSupported = 96,
    #[error("Address family not supported by protocol")]
    AddressFamilyNotSupportedByProtocol = 97,
    #[error("Address already in use")]
    AddressAlreadyInUse = 98,
    #[error("Cannot assign requested address")]
    CannotAssignRequestedAddress = 99,
    #[error("Network is down")]
    NetworkDown = 100,
    #[error("Network is unreachable")]
    NetworkUnreachable = 101,
    #[error("Network dropped connection because of reset")]
    ConnectionReset = 102,
    #[error("Software caused connection abort")]
    ConnectionAbort = 103,
    #[error("Connection reset by peer")]
    ConnectionResetByPeer = 104,
    #[error("No buffer space available")]
    NoBufferSpaceAvailable = 105,
    #[error("Transport endpoint is already connected")]
    TransportEndpointAlreadyConnected = 106,
    #[error("Transport endpoint is not connected")]
    TransportEndpointNotConnected = 107,
    #[error("Cannot send after transport endpoint shutdown")]
    CannotSendAfterTransportEndpointShutdown = 108,
    #[error("Too many references: cannot splice")]
    TooManyReferences = 109,
    #[error("Connection timed out")]
    ConnectionTimedOut = 110,
    #[error("Connection refused")]
    ConnectionRefused = 111,
    #[error("Host is down")]
    HostDown = 112,
    #[error("No route to host")]
    NoRouteToHost = 113,
    #[error("Operation already in progress")]
    OperationAlreadyInProgress = 114,
    #[error("Operation now in progress")]
    OperationNowInProgress = 115,
    #[error("Stale file handle")]
    StaleFileHandle = 116,
    #[error("Structure needs cleaning")]
    StructureNeedsCleaning = 117,
    #[error("Not a XENIX named type file")]
    NotXenixNamedTypeFile = 118,
    #[error("No XENIX semaphores available")]
    NoXenixSemaphoresAvailable = 119,
    #[error("Is a named type file")]
    NamedTypeFile = 120,
    #[error("Remote I/O error")]
    RemoteIoError = 121,
    #[error("Quota exceeded")]
    QuotaExceeded = 122,
    #[error("No medium found")]
    NoMediumFound = 123,
    #[error("Wrong medium type")]
    WrongMediumType = 124,
    #[error("Operation Canceled")]
    OperationCanceled = 125,
    #[error("Required key not available")]
    RequiredKeyNotAvailable = 126,
    #[error("Key has expired")]
    KeyExpired = 127,
    #[error("Key has been revoked")]
    KeyRevoked = 128,
    #[error("Key was rejected by service")]
    KeyRejectedByService = 129,
    #[error("Owner died")]
    OwnerDied = 130,
    #[error("State not recoverable")]
    StateNotRecoverable = 131,
    #[error("Operation not possible due to RF-kill")]
    OperationNotPossibleDueToRfKill = 132,
    #[error("Memory page has hardware error")]
    MemoryPageHardwareError = 133,
}