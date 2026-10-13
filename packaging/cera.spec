%global debug_package %{nil}
%global _build_id_links none

Name:           cera
Version:        %{cera_version}
Release:        1
Summary:        Terminal text editor with a plugin system
License:        GPL-3.0-or-later
URL:            https://github.com/kelian-4/cera
BuildArch:      %{cera_arch}
Requires:       python3

%description
Cera is a simple terminal text editor written in Rust, with a plugin
system (C, C++, Rust, JS, Python) and syntax highlighting for 70 languages.

%install
mkdir -p %{buildroot}
cp -a %{cera_stage}/. %{buildroot}/

%files
%{_bindir}/cera
%{_datadir}/cera
%{_docdir}/cera
