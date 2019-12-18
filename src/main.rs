extern crate regex;
extern crate reqwest;

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let tmp = reqwest::get("https://www.nvidia.co.jp/Download/processDriver.aspx?psid=98&pfid=764&rpf=1&osid=57&lang=en-us&ctk=0&dtid=1&dtcid=1")?.text()?;
	let driver_result = reqwest::get(&tmp)?.text()?;

	let mut re = Regex::new(
		r#"(href=")(?P<url>/content/DriverDownload-March\d{4}/confirmation.php\?.+?)(")"#,
	)?;
	let confirmation = reqwest::get(
		&[
			"https://www.nvidia.co.jp",
			&re.captures(&driver_result).unwrap()["url"],
		]
		.concat(),
	)?
	.text()?;

	re = Regex::new(
		r#"(href=")(?P<url>http://[[:alpha:]]{2}\.download\.nvidia\.com/Windows/.+?)(")"#,
	)?;
	println!("{}", &re.captures(&confirmation).unwrap()["url"]);

	Ok(())
}
