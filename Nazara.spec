#
# spec file for package specRPM_CREATION_NAME
#
# Copyright (c) 2024 SUSE LLC
#
# All modifications and additions to the file contributed by third parties
# remain the property of their copyright owners, unless otherwise agreed
# upon. The license for this file, and modifications and additions to the
# file, is the same license as for the pristine package itself (unless the
# license for the pristine package is not an Open Source License, in which
# case the license is the MIT License). An "Open Source License" is a
# license that conforms to the Open Source Definition (Version 1.9)
# published by the Open Source Initiative.

# Please submit bugfixes or comments via https://bugs.opensuse.org/

Name:           Nazara
version:        0.1.0_alpha.1
Release:        0.1
Summary:        Automated registration and update of machines and VMs in NetBox
# FIXME: Select a correct license from https://github.com/openSUSE/spec-cleaner#spdx-licenses
License:        GPL-3.0
# FIXME: use correct group, see "https://en.opensuse.org/openSUSE:Package_group_guidelines"
# Group:
URL:            https://github.com/The-Nazara-Project/Nazara
Source0:        Nazara-%{version}.tar.gz
Source1:        vendor.tar.gz
Source2:        cargo_config
BuildRequires:  git
BuildRequires:  cargo
BuildRequires:  cargo-packaging

# the name of the actual binary
%define bin_name nazara

%description

Nazara is a cli application written in Rust to register and update machines and VMs in NetBox automatically.

%prep
%autosetup -p1 -a1
install -D -m 644 %{SOURCE2} .cargo/config

%build
%{cargo_build}

%install
# manually
install -D -d -m 0755 %{buildroot}%{_bindir}
install -m 0755 %{_builddir}/%{name}-%{version}/target/release/%{bin_name} %{buildroot}%{_bindir}/%{bin_name}


%files
%{_bindir}/%{bin_name}
%license LICENSE
%doc README.md

%changelog
* Tue Oct 01 2024 Tiara Hock <tiara.dev@proton.me> - 0.0.1_pre-alpha.3
- Enable machine update
- Fix search of existing NetBox objects
- Update thanix_client dependency to v1.3.2
* Thu Sep 26 2024 Tiara Hock <tiara.dev@proton.me> - 0.1.0_pre-alpha.2
- Enable in-bulk creation of network interfaces & IP addresses
- Finish machine registration feature
- Update thanix_client dependency to v1.3.1
