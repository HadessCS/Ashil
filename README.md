### Ashil


![Ashil](ashil-logo.jpg)



## Background

"We've developed a security software named Ashil and a vulnerable program named Delta, inspired by the following story: Ashil, the lovable 11-year-old German Shepherd, passed away due to a heart attack just a month before his 11th birthday. He adored Delta, and on his final day, he strived to share all his food with Delta. The memory of Ashil will forever remain in our hearts."

Brought to you by:

<img src="https://hadess.io/wp-content/uploads/2022/04/LOGOTYPE-tag-white-.png" alt="HADESS" width="200"/>

[HADESS](https://hadess.io) performs offensive cybersecurity services through infrastructures and software that include vulnerability analysis, scenario attack planning, and implementation of custom integrated preventive projects. We organized our activities around the prevention of corporate, industrial, and laboratory cyber threats.

## Installation

```
pip install -r requirements.txt  
python main.py
```

## Command Line Options
```
          
	  --help                       Display help
	  --path  					   Required path of apk file
	  --manifest  				   Display manifest informations
	  --infra  					   Find all infra addresses included ip,domain ex. --infra ip,domain
	  --whoise  				   Whoise all infra included ip,domain ex. --whoise ip,domain
	  --output  				   Set output files ex. --output out.txt
	 
```

## Usage

### Display Manifest

[![asciicast](https://asciinema.org/a/592332.svg)](https://asciinema.org/a/592332)

### IP Whois

[![asciicast](https://asciinema.org/a/592335.svg)](https://asciinema.org/a/592335)

Example Usage:

1.Find infra(domain and ip) in sample4.apk and set output result into out.txt

```
python3 main.py --path sample4.apk --infra domain,ip --output out.txt
```


2. Investigate the Domain and IP on the APK

```
python3 main.py --path sample.apk --whois ip
```
