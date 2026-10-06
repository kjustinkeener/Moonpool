---
title: "Windows protected your PC: run the Moonpool installer anyway (SmartScreen)"
description: "Windows SmartScreen shows Windows protected your PC when you run moonpool.exe. Why it appears, how to choose More info then Run anyway, and what to check first."
---

When you run the downloaded `moonpool.exe`, Windows may show a blue box titled **Windows
protected your PC**, with the line "Microsoft Defender SmartScreen prevented an unrecognized
app from starting. Running this app might put your PC at risk."

## Why it appears

SmartScreen warns about programs that are new or that it has not seen run on many PCs.
`moonpool.exe` is not code-signed, so Windows has no publisher to trust and may show the
warning the first time. It is a reputation check, not a finding that the file is malicious.

## What to do

1. In the box, click **More info**. The publisher shows as "Unknown publisher".
2. Click **Run anyway**. The install card opens. See [Installing](/getting-started/install/).

If you want to be careful first, download only from Moonpool's official site or its GitHub
releases, and check the file name is `moonpool.exe`.

## If there is no Run anyway button

On some managed PCs the administrator turns the option off, and you will see no **Run
anyway**. Ask your administrator, or use a PC you manage. A file that came in a downloaded
zip can also carry a block: right-click the file, choose **Properties**, tick **Unblock** if
it appears, then **OK** and run it again.

## Antivirus warnings

A new unsigned exe that copies itself into your profile and replaces itself on update can
also trip antivirus software. If yours blocks or quarantines `moonpool.exe`, allow it for
the `.moonpool` folder. See [Windows](/platforms/windows/#before-you-run-it).

## See also

- [Installing](/getting-started/install/)
- [Windows](/platforms/windows/)
- [The installer shows an error](/support/troubleshooting/#the-installer-shows-an-error)
