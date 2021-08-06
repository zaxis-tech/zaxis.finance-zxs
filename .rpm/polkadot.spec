%define debug_package %{nil}

Name: zaxis
Summary: Implementation of a https://zaxis.network node in Rust based on the Substrate framework.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/zaxis"
getent group zaxis >/dev/null || groupadd -r zaxis
getent passwd zaxis >/dev/null || \
    useradd -r -g zaxis -d /home/zaxis -m -s /sbin/nologin \
    -c "User account for running zaxis as a service" zaxis
if [ ! -e "$config_file" ]; then
    echo 'ZAXIS_CLI_ARGS=""' > /etc/default/zaxis
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/zaxis.service
