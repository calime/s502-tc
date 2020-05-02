var searchIndex = JSON.parse('{\
"s502_as":{"doc":"","i":[[5,"main","s502_as","",null,[[],["exitcode",3]]],[0,"callbacks","","",null,null],[5,"eol","s502_as::callbacks","Process an assembly statement at the end of each line and…",null,[[["lexer",3]],["filter",4]]],[0,"labels","","",null,null],[5,"vis_object","s502_as::callbacks::labels","",null,[[["lexer",3]],["filter",4]]],[5,"vis_global","","",null,[[["lexer",3]],["filter",4]]],[5,"label","","Process a root label, either at the beginning of the line…",null,[[["lexer",3]],["filter",4]]],[5,"child_label","","Process a child label, either at the beginning of the line…",null,[[["lexer",3]],["filter",4]]],[0,"mnemonics","s502_as::callbacks","",null,null],[5,"insert_mnem","s502_as::callbacks::mnemonics","Attempt to insert an instruction in the machine. Fails if…",null,[[["mnemonic",4],["lexer",3]],["filter",4]]],[5,"adc","","",null,[[["lexer",3]],["filter",4]]],[5,"and","","",null,[[["lexer",3]],["filter",4]]],[5,"asl","","",null,[[["lexer",3]],["filter",4]]],[5,"bcc","","",null,[[["lexer",3]],["filter",4]]],[5,"bcs","","",null,[[["lexer",3]],["filter",4]]],[5,"beq","","",null,[[["lexer",3]],["filter",4]]],[5,"bit","","",null,[[["lexer",3]],["filter",4]]],[5,"bmi","","",null,[[["lexer",3]],["filter",4]]],[5,"bne","","",null,[[["lexer",3]],["filter",4]]],[5,"bpl","","",null,[[["lexer",3]],["filter",4]]],[5,"brk","","",null,[[["lexer",3]],["filter",4]]],[5,"bvc","","",null,[[["lexer",3]],["filter",4]]],[5,"bvs","","",null,[[["lexer",3]],["filter",4]]],[5,"clc","","",null,[[["lexer",3]],["filter",4]]],[5,"cld","","",null,[[["lexer",3]],["filter",4]]],[5,"cli","","",null,[[["lexer",3]],["filter",4]]],[5,"clv","","",null,[[["lexer",3]],["filter",4]]],[5,"cmp","","",null,[[["lexer",3]],["filter",4]]],[5,"cpx","","",null,[[["lexer",3]],["filter",4]]],[5,"cpy","","",null,[[["lexer",3]],["filter",4]]],[5,"dec","","",null,[[["lexer",3]],["filter",4]]],[5,"dex","","",null,[[["lexer",3]],["filter",4]]],[5,"dey","","",null,[[["lexer",3]],["filter",4]]],[5,"eor","","",null,[[["lexer",3]],["filter",4]]],[5,"inc","","",null,[[["lexer",3]],["filter",4]]],[5,"inx","","",null,[[["lexer",3]],["filter",4]]],[5,"iny","","",null,[[["lexer",3]],["filter",4]]],[5,"jmp","","",null,[[["lexer",3]],["filter",4]]],[5,"jsr","","",null,[[["lexer",3]],["filter",4]]],[5,"lda","","",null,[[["lexer",3]],["filter",4]]],[5,"ldx","","",null,[[["lexer",3]],["filter",4]]],[5,"ldy","","",null,[[["lexer",3]],["filter",4]]],[5,"lsr","","",null,[[["lexer",3]],["filter",4]]],[5,"nop","","",null,[[["lexer",3]],["filter",4]]],[5,"ora","","",null,[[["lexer",3]],["filter",4]]],[5,"pha","","",null,[[["lexer",3]],["filter",4]]],[5,"php","","",null,[[["lexer",3]],["filter",4]]],[5,"pla","","",null,[[["lexer",3]],["filter",4]]],[5,"plp","","",null,[[["lexer",3]],["filter",4]]],[5,"rol","","",null,[[["lexer",3]],["filter",4]]],[5,"ror","","",null,[[["lexer",3]],["filter",4]]],[5,"rti","","",null,[[["lexer",3]],["filter",4]]],[5,"rts","","",null,[[["lexer",3]],["filter",4]]],[5,"sbc","","",null,[[["lexer",3]],["filter",4]]],[5,"sec","","",null,[[["lexer",3]],["filter",4]]],[5,"sed","","",null,[[["lexer",3]],["filter",4]]],[5,"sei","","",null,[[["lexer",3]],["filter",4]]],[5,"sta","","",null,[[["lexer",3]],["filter",4]]],[5,"stx","","",null,[[["lexer",3]],["filter",4]]],[5,"sty","","",null,[[["lexer",3]],["filter",4]]],[5,"tax","","",null,[[["lexer",3]],["filter",4]]],[5,"tay","","",null,[[["lexer",3]],["filter",4]]],[5,"tsx","","",null,[[["lexer",3]],["filter",4]]],[5,"txa","","",null,[[["lexer",3]],["filter",4]]],[5,"txs","","",null,[[["lexer",3]],["filter",4]]],[5,"tya","","",null,[[["lexer",3]],["filter",4]]],[5,"dfb","","",null,[[["lexer",3]],["filter",4]]],[5,"dfw","","",null,[[["lexer",3]],["filter",4]]],[5,"hlt","","",null,[[["lexer",3]],["filter",4]]],[5,"sct","","",null,[[["lexer",3]],["filter",4]]],[0,"operands","s502_as::callbacks","",null,null],[5,"byte_high","s502_as::callbacks::operands","Take the high byte of the preceding reference.",null,[[["lexer",3]],["filter",4]]],[5,"byte_low","","Take the low byte of the preceding reference.",null,[[["lexer",3]],["filter",4]]],[5,"acc","","Accept the accumulator operand. Fails if not the first in…",null,[[["lexer",3]],["filter",4]]],[5,"xreg","","",null,[[["lexer",3]],["filter",4]]],[5,"yreg","","",null,[[["lexer",3]],["filter",4]]],[5,"imme","","",null,[[["lexer",3]],["filter",4]]],[5,"number","","",null,[[["lexer",3]],["filter",4]]],[5,"lparen","","",null,[[["lexer",3]],["filter",4]]],[5,"rparen","","",null,[[["lexer",3]],["filter",4]]],[5,"comma","","",null,[[["lexer",3]],["filter",4]]],[0,"ir","s502_as","",null,null],[3,"Program","s502_as::ir","",null,null],[12,"line","","",0,null],[12,"sections","","",0,null],[12,"active","","",0,null],[12,"vis","","Visibility for current label.",0,null],[12,"start_line","","",0,null],[12,"ins","","",0,null],[12,"op","","",0,null],[12,"err","","",0,null],[3,"Section","","",null,null],[12,"code","","",1,null],[12,"labels","","",1,null],[12,"references","","",1,null],[12,"size","","",1,null],[12,"last_parent","","",1,null],[3,"Label","","",null,null],[12,"vis","","",2,null],[12,"name","","",2,null],[12,"num_children","","",2,null],[12,"offset","","",2,null],[3,"Reference","","",null,null],[12,"parent","","",3,null],[12,"child","","",3,null],[12,"offset","","",3,null],[12,"which_byte","","",3,null],[4,"ByteSelect","","",null,null],[13,"Both","","",4,null],[13,"High","","",4,null],[13,"Low","","",4,null],[4,"Visibility","","",null,null],[13,"Hidden","","",5,null],[13,"Object","","",5,null],[13,"Global","","",5,null],[4,"OpState","","",null,null],[13,"StartInd","","Beginning of an indirect operand `(`.",6,null],[13,"StartImme","","Beginning of an immediate operand `#`.",6,null],[13,"Plain","","Undecorated operand, may become absolute, zeropage, or…",6,null],[13,"MaybeInd","","Operand will be indirect `(val`.",6,null],[13,"Idx","","Operand will be indexed `val,`",6,null],[13,"Acc","","Accumulator operand",6,null],[13,"AbsX","","",6,null],[13,"AbsY","","",6,null],[13,"Imme","","",6,null],[13,"Ind","","May be abs,x or zpg,x depending on which_byte",6,null],[13,"Xind","","",6,null],[13,"IndY","","",6,null],[13,"ZpgX","","",6,null],[13,"ZpgY","","",6,null],[13,"Impl","","",6,null],[4,"IndOp","","The type of operand that can appear inside parentheses.",null,null],[13,"Other","","Either `($HHLL` or `$(LL`.",7,null],[13,"Xind","","Currently has the form `($LL,x`.",7,null],[13,"XindComma","","Currently has the form `($LL,`.",7,null],[13,"IndY","","Currently has the form `($LL)`.",7,null],[13,"IndYComma","","Currently has the form `($LL),`.",7,null],[4,"OpVal","","The type the operand\'s value can be.",null,null],[13,"Byte","","",8,null],[13,"Word","","",8,null],[13,"Ref","","",8,null],[4,"Mnemonic","","A mnemonic, both for instructions and directives.",null,null],[13,"Adc","","",9,null],[13,"And","","",9,null],[13,"Asl","","",9,null],[13,"Bcc","","",9,null],[13,"Bcs","","",9,null],[13,"Beq","","",9,null],[13,"Bit","","",9,null],[13,"Bmi","","",9,null],[13,"Bne","","",9,null],[13,"Bpl","","",9,null],[13,"Brk","","",9,null],[13,"Bvc","","",9,null],[13,"Bvs","","",9,null],[13,"Clc","","",9,null],[13,"Cld","","",9,null],[13,"Cli","","",9,null],[13,"Clv","","",9,null],[13,"Cmp","","",9,null],[13,"Cpx","","",9,null],[13,"Cpy","","",9,null],[13,"Dec","","",9,null],[13,"Dex","","",9,null],[13,"Dey","","",9,null],[13,"Eor","","",9,null],[13,"Inc","","",9,null],[13,"Inx","","",9,null],[13,"Iny","","",9,null],[13,"Jmp","","",9,null],[13,"Jsr","","",9,null],[13,"Lda","","",9,null],[13,"Ldx","","",9,null],[13,"Ldy","","",9,null],[13,"Lsr","","",9,null],[13,"Nop","","",9,null],[13,"Ora","","",9,null],[13,"Pha","","",9,null],[13,"Php","","",9,null],[13,"Pla","","",9,null],[13,"Plp","","",9,null],[13,"Rol","","",9,null],[13,"Ror","","",9,null],[13,"Rti","","",9,null],[13,"Rts","","",9,null],[13,"Sbc","","",9,null],[13,"Sec","","",9,null],[13,"Sed","","",9,null],[13,"Sei","","",9,null],[13,"Sta","","",9,null],[13,"Stx","","",9,null],[13,"Sty","","",9,null],[13,"Tax","","",9,null],[13,"Tay","","",9,null],[13,"Tsx","","",9,null],[13,"Txa","","",9,null],[13,"Txs","","",9,null],[13,"Tya","","",9,null],[13,"Dfb","","",9,null],[13,"Dfw","","",9,null],[13,"Hlt","","",9,null],[13,"Sct","","",9,null],[4,"AddressMode","","The address mode parsed.",null,null],[13,"Acc","","",10,null],[13,"Abs","","",10,null],[13,"AbsX","","",10,null],[13,"AbsY","","",10,null],[13,"Imme","","",10,null],[13,"Impl","","",10,null],[13,"Ind","","",10,null],[13,"Xind","","",10,null],[13,"IndY","","",10,null],[13,"Zpg","","",10,null],[13,"ZpgX","","",10,null],[13,"ZpgY","","",10,null],[11,"destruct","","Turn an opstate into an AddressMode because enum map only…",6,[[],["option",4]]],[0,"opcodes","s502_as","",null,null],[3,"OPCODES","s502_as::opcodes","Lookup opcode based on mnemonic and address mode. None…",null,null],[12,"__private_field","","",11,null],[0,"output","s502_as","",null,null],[5,"asm","s502_as::output","Assemble a file and output its object.",null,[[["string",3],["option",4]]]],[5,"create_object","","Output a program to an object file.",null,[[["hashmap",3],["section",3],["string",3]],["result",6]]],[0,"token","s502_as","",null,null],[4,"Token","s502_as::token","",null,null],[13,"Adc","","",12,null],[13,"And","","",12,null],[13,"Asl","","",12,null],[13,"Bcc","","",12,null],[13,"Bcs","","",12,null],[13,"Beq","","",12,null],[13,"Bit","","",12,null],[13,"Bmi","","",12,null],[13,"Bne","","",12,null],[13,"Bpl","","",12,null],[13,"Brk","","",12,null],[13,"Bvc","","",12,null],[13,"Bvs","","",12,null],[13,"Clc","","",12,null],[13,"Cld","","",12,null],[13,"Cli","","",12,null],[13,"Clv","","",12,null],[13,"Cmp","","",12,null],[13,"Cpx","","",12,null],[13,"Cpy","","",12,null],[13,"Dec","","",12,null],[13,"Dex","","",12,null],[13,"Dey","","",12,null],[13,"Eor","","",12,null],[13,"Inc","","",12,null],[13,"Inx","","",12,null],[13,"Iny","","",12,null],[13,"Jmp","","",12,null],[13,"Jsr","","",12,null],[13,"Lda","","",12,null],[13,"Ldx","","",12,null],[13,"Ldy","","",12,null],[13,"Lsr","","",12,null],[13,"Nop","","",12,null],[13,"Ora","","",12,null],[13,"Pha","","",12,null],[13,"Php","","",12,null],[13,"Pla","","",12,null],[13,"Plp","","",12,null],[13,"Rol","","",12,null],[13,"Ror","","",12,null],[13,"Rti","","",12,null],[13,"Rts","","",12,null],[13,"Sbc","","",12,null],[13,"Sec","","",12,null],[13,"Sed","","",12,null],[13,"Sei","","",12,null],[13,"Sta","","",12,null],[13,"Stx","","",12,null],[13,"Sty","","",12,null],[13,"Tax","","",12,null],[13,"Tay","","",12,null],[13,"Tsx","","",12,null],[13,"Txa","","",12,null],[13,"Txs","","",12,null],[13,"Tya","","",12,null],[13,"Dfb","","",12,null],[13,"Dfw","","",12,null],[13,"Hlt","","",12,null],[13,"Sct","","",12,null],[13,"A","","",12,null],[13,"X","","",12,null],[13,"Y","","",12,null],[13,"Comma","","",12,null],[13,"Imme","","",12,null],[13,"Lparen","","",12,null],[13,"Rparen","","",12,null],[13,"ByteHi","","",12,null],[13,"ByteLo","","",12,null],[13,"VisObj","","",12,null],[13,"VisGlobal","","",12,null],[13,"Num","","",12,null],[13,"Ident","","",12,null],[13,"Eol","","",12,null],[13,"Error","","",12,null],[11,"from","s502_as::ir","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","s502_as::opcodes","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","s502_as::token","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"clone","s502_as::ir","",2,[[],["label",3]]],[11,"clone","","",3,[[],["reference",3]]],[11,"clone","","",4,[[],["byteselect",4]]],[11,"clone","","",5,[[],["visibility",4]]],[11,"clone","","",8,[[],["opval",4]]],[11,"clone","","",10,[[],["addressmode",4]]],[11,"default","","",0,[[]]],[11,"default","","",1,[[]]],[11,"eq","","",4,[[["byteselect",4]]]],[11,"eq","","",10,[[["addressmode",4]]]],[11,"deref","s502_as::opcodes","",11,[[],["enummap",3]]],[11,"lex","s502_as::token","",12,[[["lexer",3]]]],[11,"slice","s502_as::ir","",9,[[]]],[11,"slice_mut","","",9,[[]]],[11,"from_usize","","",9,[[]]],[11,"to_usize","","",9,[[]]],[11,"from_function","","",9,[[["fnmut",8]]]],[11,"slice","","",10,[[]]],[11,"slice_mut","","",10,[[]]],[11,"from_usize","","",10,[[]]],[11,"to_usize","","",10,[[]]],[11,"from_function","","",10,[[["fnmut",8]]]],[11,"initialize","s502_as::opcodes","",11,[[]]]],"p":[[3,"Program"],[3,"Section"],[3,"Label"],[3,"Reference"],[4,"ByteSelect"],[4,"Visibility"],[4,"OpState"],[4,"IndOp"],[4,"OpVal"],[4,"Mnemonic"],[4,"AddressMode"],[3,"OPCODES"],[4,"Token"]]},\
"s502_ln":{"doc":"","i":[[5,"main","s502_ln","",null,[[]]],[0,"formats","","In addition to the Tilt and assembly files, the S502…",null,null],[3,"Section","s502_ln::formats","",null,null],[12,"code","","",0,null],[12,"labels","","",0,null],[12,"references","","",0,null],[12,"size","","",0,null],[12,"last_parent","","",0,null],[3,"Label","","",null,null],[12,"vis","","",1,null],[12,"name","","",1,null],[12,"num_children","","",1,null],[12,"offset","","",1,null],[3,"Reference","","",null,null],[12,"parent","","",2,null],[12,"child","","",2,null],[12,"offset","","",2,null],[12,"which_byte","","",2,null],[4,"ByteSelect","","",null,null],[13,"Both","","",3,null],[13,"High","","",3,null],[13,"Low","","",3,null],[4,"Visibility","","",null,null],[13,"Hidden","","",4,null],[13,"Object","","",4,null],[13,"Global","","",4,null],[0,"input","s502_ln","",null,null],[11,"from","s502_ln::formats","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"clone","","",1,[[],["label",3]]],[11,"clone","","",2,[[],["reference",3]]],[11,"clone","","",3,[[],["byteselect",4]]],[11,"clone","","",4,[[],["visibility",4]]],[11,"eq","","",3,[[["byteselect",4]]]]],"p":[[3,"Section"],[3,"Label"],[3,"Reference"],[4,"ByteSelect"],[4,"Visibility"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);