# Generated by rust2rpm 22
%bcond_without check

%global crate album-creator

Name:           rust-album-creator
Version:        0.1.1
Release:        %autorelease
Summary:        Tool to batch process images

License:        MIT
URL:            https://crates.io/crates/album-creator
Source:         %{crates_source}

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging >= 21

%global _description %{expand:
Tool to batch process images.}

%description %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
%license LICENSE
%doc README.md
%{_bindir}/album-creator

%prep
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%changelog
%autochangelog
