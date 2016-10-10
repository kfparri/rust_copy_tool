# Rusty Copy

A simple scriptable command line utility to universally copy files.

## What does it do?
Rusty is a simple application that looks at a directory for a configuration file (transfer.json), this file will have a name of a file, a destination where that file should go, how often it should run and the next run time of the copy.  

## Why would I need this?
Sometimes, you need to be able to move files around based on a schedule.  If you're like most savvy computer users, you could simply write a little script to move/copy the file and create a scheduled task to have the job run whenever you want.  But what if you are supporting users that can't do that?  Or what if you have a server that generates files that need to be published to a specific location on a reqular basis (such as a report server)?  Well you could create a job for each file you need to copy, or create a single job that just moves all the files, but that could be either very tedious or inefficient.

What if you had a single job that could run as often as you want and simply read in a file specific configuration for each file you wanted to copy?  Enter Rusty!  This application can be scheduled to run and check all folders for the configuration file and only transfer the files if necessary.

## Why did I build this?
I am a student at the University of Nebraska Lincoln and part of my intro to CSCE, I had to learn a new language.  I decided to learn the Rust programming language.  I also had a need for a simple to configure job to move reports from a report server to a web server without overcomplicating the process by including FTP or overly complicated scripting code.

## How do I use it?
`rusty_copy <Path to transfer folder>`

The folder should have the following layout:


