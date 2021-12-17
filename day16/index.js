const input =
  "6051639005B56008C1D9BB3CC9DAD5BE97A4A9104700AE76E672DC95AAE91425EF6AD8BA5591C00F92073004AC0171007E0BC248BE0008645982B1CA680A7A0CC60096802723C94C265E5B9699E7E94D6070C016958F99AC015100760B45884600087C6E88B091C014959C83E740440209FC89C2896A50765A59CE299F3640D300827902547661964D2239180393AF92A8B28F4401BCC8ED52C01591D7E9D2591D7E9D273005A5D127C99802C095B044D5A19A73DC0E9C553004F000DE953588129E372008F2C0169FDB44FA6C9219803E00085C378891F00010E8FF1AE398803D1BE25C743005A6477801F59CC4FA1F3989F420C0149ED9CF006A000084C5386D1F4401F87310E313804D33B4095AFBED32ABF2CA28007DC9D3D713300524BCA940097CA8A4AF9F4C00F9B6D00088654867A7BC8BCA4829402F9D6895B2E4DF7E373189D9BE6BF86B200B7E3C68021331CD4AE6639A974232008E663C3FE00A4E0949124ED69087A848002749002151561F45B3007218C7A8FE600FC228D50B8C01097EEDD7001CF9DE5C0E62DEB089805330ED30CD3C0D3A3F367A40147E8023221F221531C9681100C717002100B36002A19809D15003900892601F950073630024805F400150D400A70028C00F5002C00252600698400A700326C0E44590039687B313BF669F35C9EF974396EF0A647533F2011B340151007637C46860200D43085712A7E4FE60086003E5234B5A56129C91FC93F1802F12EC01292BD754BCED27B92BD754BCED27B100264C4C40109D578CA600AC9AB5802B238E67495391D5CFC402E8B325C1E86F266F250B77ECC600BE006EE00085C7E8DF044001088E31420BCB08A003A72BF87D7A36C994CE76545030047801539F649BF4DEA52CBCA00B4EF3DE9B9CFEE379F14608";

const hexBin = {
  0: "0000",
  1: "0001",
  2: "0010",
  3: "0011",
  4: "0100",
  5: "0101",
  6: "0110",
  7: "0111",
  8: "1000",
  9: "1001",
  A: "1010",
  B: "1011",
  C: "1100",
  D: "1101",
  E: "1110",
  F: "1111",
};

function part1(input) {
  const binStr = hexToBin(input);
  const result = parse(binStr);
  console.log(sumVersions(result));
}

function part2(input) {
  const binStr = hexToBin(input);
  const result = parse(binStr);
  // console.log(JSON.stringify(result, null, "  "));
  console.log(evaluate(result));
}

function parse(str) {
  const { version, typeId } = parseHeader(str);

  let packets;

  if (typeId === 4) {
    packets = parseLiteral(str);
  } else {
    packets = parseOperator(str);
  }

  return { version, typeId, packets };
}

const hexToBin = (str) => str.split("").flatMap((x) => hexBin[x].split(""));
const bin = (str) => parseInt(str, 2);
const pop = (str, count) => str.splice(0, count).join("");

const parseHeader = (str) => {
  const version = bin(pop(str, 3));
  const typeId = bin(pop(str, 3));

  return { version, typeId };
};

const parseLiteral = (str) => {
  let literal = "";
  let firstBit = "";
  do {
    const group = pop(str, 5);
    firstBit = group.slice(0, 1);
    literal += group.slice(1);
  } while (firstBit === "1");

  return bin(literal);
};

const parseOperator = (str) => {
  const lengthBit = pop(str, 1);
  const subPackets = [];

  if (lengthBit === "0") {
    const totalLength = bin(pop(str, 15));
    const subPacketBits = pop(str, totalLength).split("");
    while (subPacketBits.length) {
      subPackets.push(parse(subPacketBits));
    }
  } else {
    let numberOfSubpackets = bin(pop(str, 11));

    while (numberOfSubpackets--) {
      subPackets.push(parse(str));
    }
  }

  return subPackets;
};

const types = {
  // sum
  0: (packets) => packets.reduce((acc, next) => acc + evaluate(next), 0),
  // product
  1: (packets) => packets.reduce((acc, next) => acc * evaluate(next), 1),
  // min
  2: (packets) => Math.min(...packets.map(evaluate)),
  // max
  3: (packets) => Math.max(...packets.map(evaluate)),
  // literal
  4: (value) => value,
  // greater than
  5: ([a, b]) => (evaluate(a) > evaluate(b) ? 1 : 0),
  // less than
  6: ([a, b]) => (evaluate(a) < evaluate(b) ? 1 : 0),
  // equal
  7: ([a, b]) => (evaluate(a) === evaluate(b) ? 1 : 0),
};

const evaluate = (packet) => types[packet.typeId](packet.packets);

const sumVersions = (packet) => {
  if (!Array.isArray(packet.packets)) return packet.version;
  const vals = packet.packets.map((p) => sumVersions(p));
  return vals.reduce((acc, v) => acc + v, packet.version);
};

part2(input);
