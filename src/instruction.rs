#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    AddWithCarry,               //AND add with carry
    AndWithAccumulator,         //AND and (with accumulator)
    ArithmeticShiftLeft,        //ASL arithmetic shift left
    BranchIfCarryClear,         //BCC branch on carry clear
    BranchIfCarrySet,           //BCS branch on carry set
    BranchIfEqual,              //BEQ branch on equal (zero set)
    BitSet,                     //BIT bit test
    BranchIfMinus,              // BMI branch on minus (negative set)
    BranchIfNotEqual,           //BNE branch on not equal (zero clear)
    BranchIfPlus,               // BPL branch on plus (negative clear)
    Break,                      //BRK break / interrupt
    BranchIfOverflowClear,      //BVC branch on overflow clear
    BranchIfOverflowSet,        //BVS branch on overflow set
    ClearCarry,                 //CLC clear carry
    ClearDecimal,               //CLD clear decimal
    ClearInterrupt,             //CLI clear interrupt disable
    ClearOverflow,              //CLV clear overflow
    CompareWithAccumulator,     //CMP compare (with accumulator)
    CompareWithX,               //CPX compare with X
    CompareWithY,               //CPY compare with Y
    Decrement,                  //DEC decrement
    DecrementX,                 //DEX decrement X
    DecrementY,                 //DEY decrement Y
    ExclusiveOrWithAccumulator, //EOR exclusive or (with accumulator)
    Increment,                  //INC increment
    IncrementX,                 //INX increment X
    IncrementY,                 //INY increment Y
    Jump,                       //JMP jump
    JumpSubroutine,             //JSR jump subroutine
    LoadAccumulator,            //LDA load accumulator
    LoadX,                      //LDX load X
    LoadY,                      //LDY load Y
    LogicalShiftRight,          //LSR logical shift right
    NoOperation,                //NOP no operation
    OrWithAccumulator,          //ORA or with accumulator
    PushAccumulator,            //PHA push accumulator
    PushProcessorStatus,        //PHP push processor status (SR)
    PullAccumulator,            //PLA pull accumulator
    PullProcessorStatus,        //PLP pull processor status (SR)
    RotateLeft,                 //ROL rotate left
    RotateRight,                //ROR rotate right
    ReturnFromInterrupt,        //RTI return from interrupt
    ReturnFromSubroutine,       //RTS return from subroutine
    SubtractWithCarry,          //SBC subtract with carry
    SetCarry,                   //SEC set carry
    SetDecimal,                 //SED set decimal
    SetInterruptDisable,        //SEI set interrupt disable
    StoreAccumulator,           //STA store accumulator
    StoreX,                     //STX store X
    StoreY,                     //STY store Y
    TransferAccumulatorToX,     //TAX transfer accumulator to X
    TransferAccumulatorToY,     //TAY transfer accumulator to Y
    TransferStackPointerToX,    //TSX transfer stack pointer to X
    TransferXToAccumulator,     //TXA transfer X to accumulator
    TransferXToStackPointer,    //TXS transfer X to stack pointer
    TransferYToAccumulator,     //TYA transfer Y to accumulator
}
