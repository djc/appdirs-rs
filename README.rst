A port of ActiveState's venerable appdirs package
=================================================

.. image:: https://travis-ci.org/djc/appdirs-rs.svg
   :target: https://travis-ci.org/djc/appdirs-rs

.. image:: https://ci.appveyor.com/api/projects/status/1aklr7uk1av5cas7/branch/master?svg=true
   :target: https://ci.appveyor.com/project/djc/appdirs-rs

`ActiveState`_ first shared the source of their Python `appdirs`_ package in
2010, before which it had been developed as part of `Komodo`_. It has since
evolved at slow intervals, and seems to be a go-to package in the Python
ecosystem. This package is a Rust port of the Python code.

It exposes the same six primitives:

* ``user_data_dir()``
* ``site_data_dir()``
* ``user_config_dir()``
* ``site_config_dir()``
* ``user_cache_dir()``
* ``user_log_dir()``

Windows Vista and later, macOS and other Unices are tested. Android support
should be coming soon; iOS needs some more investigation. The following
documents have been guiding development:

* Microsoft Windows `knowledge base entry`_
* Apple's `File System Programming Guide`_
* The `XDG Base Directory Specification`_

.. _ActiveState: http://www.activestate.com/
.. _appdirs: https://github.com/ActiveState/appdirs
.. _Komodo: http://komodoide.com/
.. _knowledge base entry: http://support.microsoft.com/default.aspx?scid=kb;en-us;310294#XSLTH3194121123120121120120
.. _File System Programming Guide: https://developer.apple.com/library/mac/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html
.. _XDG Base Directory Specification: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
