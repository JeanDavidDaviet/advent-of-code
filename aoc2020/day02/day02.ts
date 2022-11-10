const decoder = new TextDecoder('utf-8');
const input = await Deno.readFile('input.txt');
const lines = decoder.decode(input).trim().split("\n");

interface PasswordPolicy {
  min: number;
  max: number;
  byte: string;
  password: string;
}

const lineToPasswordPolicy = (line: string) : PasswordPolicy => {
  const [policy, password] = line.split(': ') as [string, string];
  const [minmax, byte] = policy.split(' ') as [string, string];
  const [min, max] = minmax.split('-').map(number => parseInt(number, 10)) as [number, number];
  
  return {
    min,
    max,
    byte,
    password
  }
}

const verifyPasswordPart1 = (passwordPolicy: PasswordPolicy) : boolean => {
  const regexp = new RegExp(`${passwordPolicy.byte}{1}`, 'g');
  const matches = passwordPolicy.password.match(regexp) || [];
  
  return matches.length >= passwordPolicy.min && matches.length <= passwordPolicy.max;
}

const verifyPasswordPart2 = (passwordPolicy: PasswordPolicy) : boolean => {
  return (passwordPolicy.password[passwordPolicy.min - 1] === passwordPolicy.byte && passwordPolicy.password[passwordPolicy.max - 1] != passwordPolicy.byte) ||
    (passwordPolicy.password[passwordPolicy.min - 1] !== passwordPolicy.byte && passwordPolicy.password[passwordPolicy.max - 1] == passwordPolicy.byte)
}

const passwordPolicies = lines.map(lineToPasswordPolicy);
const verifiedPasswordsPart1 = passwordPolicies.filter(verifyPasswordPart1);
const verifiedPasswordsPart2 = passwordPolicies.filter(verifyPasswordPart2);


// ANSWER
console.log(verifiedPasswordsPart1.length);
console.log(verifiedPasswordsPart2.length);

export {}
