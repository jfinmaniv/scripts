#!/bin/sh

# Rsync wrapper used primarily to sync local copies of networked drives

# Typical rsync options are -av or --archive --verbose
# The --archive option is itself a bundle of other options:
# 	--recursive
# 	--links 	copy symlinks as symlinks
# 	--perms 
# 	--times 	preserve modification times
# 	--group
# 	--owner
# 	--devices
# 	--specials 	preserve special files, e.g. named sockets
 
# We have ommitted the --links, --devices, and --specials options since msys2
# does not support symlinks and the networked drives do not contain any device
# or special files that we are interested in. We have added the --progress
# option to add a progress bar and the --human-readable option so that file
# sizes are reported in k, M, and G rather than in bytes.

rsync \
	--recursive \
	--perms \
	--times \
	--group \
	--owner \
	--verbose \
	--progress \ 		
	--human-readable \
	"$@"
