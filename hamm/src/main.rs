fn main() {
	let a: &str = "GGTTGTGTCTTGCCCGGATGCGCTTAGGGGCGCCCAGGGTGCTTTTAGTGTCGTCATATACGTTTAGACCCGGACCTCCCCAGATTTGTAAAACGACGGGACCAGGCCATGCACGGTTCCTACCCAGTATTGTATGCTAGGTTTGCGGAATTACGAGCTCTTGAGTAGTTAATAAAGGTCTCACAGTGTTCCCGGTAGCGCCAATCTGGGCCTCTACTGGGAGCTCTGCTAGTTGTGCCAGCAATGGAGCACGCTGATGTGAGAGTGATCAATCGGGGAAATGAGCAGTCGTATTATAAGCTTGAGTCGAAAGTCGTCTACGGCAATTTAAAGCATATGAGTAAAGAGAGTCTTCAACGGTAGTATGTGCACAATCACCTTGACCTTTTTTTACCCTGCGCAAGCGCGTGCTGTACGGGGCGTCTCGTATCACTTGAGGACCGGGAACAGACGGCGGCCACGACAGATGTTTCGCAGTCTGGATCCTCGGTACCTTGTTTGGATGGAGGGCCCTTTGATTGGGTATCCCCTGAGACAATGCCTCCCACTAGGCCTGTCTGTAGCTAGCAGCAACTTTACACAACCGACTTAATTGTCCATAGGCATGGTGTTACTTACGATAAGATTATAATTTTGCGGCGATTCGAAGTACTGCCTCCACAACTTCCAACTGCATTTCGCCCTCCTCTGAAAGCCCCCACGTCGTTGGTTAGAGAAAGAAATGGCTGACCCACGCCGCCACCCATATCGATTTGATAATTGGCATCTAGGTTACAGACGTGTCGCCAGGCTATCGGCCTAGCCAGTTCGAACGGTGACTTTTATCCCTAAAAACGAGACCCGAACAATATTCGCAGTCCTCGAGTCATCTTGTGTGACATGTATTACTCAAGCCGTATCAGCACCTAGGGTAACACGTGGAACAGACCCGACTTAGGTATTCTACTC";
	let b: &str = "GTTTGTGAACGGGTCGTACGACGTCAGGAAACCGATGCGTGAGTGGAGTGGCGACATATTAGCCTATAAACGCACCTACCCGAAGCTGAATGACTATGTATGGCGAGAATGGGAGATAAAAATGCAGTGTTAGATTCCACCTACGAAGAACTACGGTTCCTGGATCTGCTACTACGACTTGTATAGTAATCGCGCTAGCAACAATAGGGTGGGTGAACGGAGGATCTCATAGTCGTAACAGCATAGCGGCACTATGTGGAGTGATTGTCGAACCGGCGACATTATGGCGCATCCAAATAGAGTGCGTACACCGTATTCCTTCGGAAAAGAATGGCTCGGAGAAGGAAGACTGGGCGACGTTGGTAGTTGTTCATTTCGACACAATGTGTTAAAACTGTATCACGGCACAAGTATTTGGCGAGTACTATCTCATCTATGCCCCTAAGCCAGAAGACATCGAAGCCTTAAGGGTCGCCGGCTTCATGGTCGCGGGCTTCATTTCGTGCAACTATCTCTGAGTCGGTATAGATGAAGGGTATTCCTCAAGTAAGTACGGTTCTATGATAAAAAGAACATTGTTGACCCGACTTAAGTTTTTATACTCAAAGGGTTACTCACCACAAGATGATAGATATAGGGAGAGTATAAGGAACCGTCCGATATCTCCCGTCGTCATCTCGTCGTCCTCGCAGAAAGCGGTCAACCAAGGACAGAGTAATACCCGGAATGATGACTTTTAAAAGCAAGCCCTATTTAGACTTGGCAGCCAGACAATTGACCTGGGCACAGCATAGATGACAAACAATTTTGAACCGACGGATTGGGCCCTACCAACCAAAGTCGTACGGGATACGGGGGCCTTGATCCTCCTTGGGAGATCAAGCTCACCCTACCCCGTGGAACCGGATGGTTAACAAGTGGAAAAGAGGCGTCTTCGGTACTCCGCGT";
	println!("{}",hamm(&a,&b));
}

fn hamm(s: &str, t: &str) -> usize {
	let n: i32 = *&s.len() as i32;
	let mut count: usize = 0;
	let s1: Vec<_> = s.chars().collect();
	let s2: Vec<_> = t.chars().collect();
	(0..n).for_each(|i| {
		if s1[i as usize] != s2[i as usize] {
			count += 1;
		}
	});
	count
}
