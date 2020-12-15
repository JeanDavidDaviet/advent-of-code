const getLinesFromFile = async (path: string) : Promise<string[]> => {
  const decoder = new TextDecoder('utf-8');
  try {
    const input = decoder.decode(await Deno.readFile(path));
    const lines = input.trim().split('\n');
    return lines;
  }
  catch {
    console.log('No input file found');
    Deno.exit(1);
  }
}

export {
  getLinesFromFile
}
