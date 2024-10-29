#needsrootforbuild
%global __cargo_skip_build 0
%global _debugsource_packages 1
%global _debuginfo_subpackages 1
%define _unpackaged_files_terminate_build 0

Name:           hmir
Version:        1.0.0
Release:        1
Summary:        Host management in rust


License:        MulanPSL-2.0
URL:            https://gitee.com/openeuler/hmir
Source0:        https://gitee.com/openeuler/hmir/releases/download/v%{version}/%{name}-%{version}.tar.gz


ExclusiveArch:  x86_64 aarch64

Requires:       pixman
Requires:       pixman-devel

%ifarch x86_64
Requires:       edk2-ovmf
%endif

%ifarch aarch64
Requires:       dtc
Requires:       edk2-aarch64
BuildRequires:  dtc-devel
%endif

BuildRequires:  cargo
BuildRequires:  cyrus-sasl
BuildRequires:  cyrus-sasl-devel
BuildRequires:  gcc
BuildRequires:  libcap
BuildRequires:  libcap-devel
BuildRequires:  libcap-ng
BuildRequires:  libcap-ng-devel
BuildRequires:  libseccomp
BuildRequires:  libseccomp-devel
BuildRequires:  pixman
BuildRequires:  pixman-devel
BuildRequires:  rust
BuildRequires:  rust-packaging

%ifarch x86_64
%define rust_gnu_target x86_64-unknown-linux-gnu
%define rust_musl_target x86_64-unknown-linux-musl
%endif
%ifarch aarch64
%define rust_gnu_target aarch64-unknown-linux-gnu
%define rust_musl_target aarch64-unknown-linux-musl
%endif

%define _cargo /usr/bin/env CARGO_HOME=.cargo RUSTC_BOOTSTRAP=1 /usr/bin/cargo

%description
Based on Rust programming language, StratoVirt is lightweight, efficient and safe. StratoVirt reduces memory resource consumption and improves VM startup speed while retains isolation capability and security capability of traditional virtualization. StratoVirt works with iSulad and Kata container, and can be integrated in Kubernetes ecosystem perfectly. The current version can be applied to microservices or serverless scenarios. StratoVirt reserves interface and design for importing more features, even standard virtualization.

Summary:        %{summary}

%files       -n stratovirt
%defattr(-,root,root,-)
%{_bindir}/stratovirt
%{_libdir}/stratovirt/static/stratovirt

%prep
%autosetup -p1

%cargo_prep

%cargo_generate_buildrequires

%build
sed -i '/\[source.crates-io\]/{n;d}' ./.cargo/config
sed -i '/\[source.local-registry\]/{n;d}' ./.cargo/config
sed -i '/\[source.local-registry\]/a directory = "vendor"' ./.cargo/config
sed -i '/^rustflags/d' ./.cargo/config
sed -i '/\[build\]/arustflags = \["-Copt-level=3", "-Cdebuginfo=2", "-Clink-arg=-Wl,-z,relro,-z,now", "-Ccodegen-units=1", "--cap-lints=warn", \]' ./.cargo/config

sed -i '$a\[profile.release\]' ./.cargo/config
sed -i '$adebug = true' ./.cargo/config

%ifarch aarch64
sed -i 's/rustflags = \[/&"-Clink-arg=-lgcc", /' ./.cargo/config
%endif

%{_cargo} build --release -Z avoid-dev-deps --target=%{rust_musl_target} --features "boot_time pvpanic demo_device vnc vnc_auth ramfb virtio_gpu trace_to_logger trace_to_ftrace trace_to_hitrace"

sed -i 's/rustflags = \[/&"-Clink-arg=-lpixman-1", /' ./.cargo/config
%{_cargo} build --release -Z avoid-dev-deps --target=%{rust_gnu_target} --features "boot_time pvpanic demo_device vnc vnc_auth ramfb virtio_gpu trace_to_logger trace_to_ftrace trace_to_hitrace"

%check
RUST_BACKTRACE=1 cargo test --workspace --exclude mod_test -- --nocapture --test-threads=1

%install
rm -rf %{buildroot}
install -d %{buildroot}%{_bindir}
install -D -m555 ./target/%{rust_gnu_target}/release/stratovirt %{buildroot}%{_bindir}

install -d %{buildroot}%{_libdir}/stratovirt/static
install -D -m555 ./target/%{rust_musl_target}/release/stratovirt %{buildroot}%{_libdir}/stratovirt/static

%changelog
* Tue Jul 30 2024 xufei <xufei30@huawei.com> - 2.4.0-2
- set debug is true for build debug package
- add rust to BuildRequires

* Fri May 10 2024 wenyuanlau <liuwenyuan9@huawei.com> 2.4.0-1
- Update to StratoVirt 2.4.0

* Mon Sep 11 2023 yezengruan <yezengruan@huawei.com> 2.2.0-16
- Add README.md

* Fri Sep  8 2023 ganqixin <ganqixin@huawei.com> - 2.2.0-15
- Delete -static-pie for musl

* Fri Sep  1 2023 Yan Wang <wangyan122@huawei.com> - 2.2.0-14
- Disable VIRTIO_F_ACCESS_PLATFORM feature in vsock

* Sat Mar 11 2023 liuxiangdong <liuxiangdong5@huawei.com> - 2.2.0-13
- add -static-pie for musl

* Tue Mar 07 2023 yezengruan <yezengruan@huawei.com> - 2.2.0-12
- Pack vhost_user_fs into rpm package

* Mon Mar 06 2023 zhukeqian <zhukeqian1@huawei.com> - 2.2.0-11
- Update to StratoVirt 2.2.0

* Thu Dec 01 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-10
- Update to StratoVirt 2.2.0-rc6

* Sun Oct 09 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-9
- Unified license name specification

* Tue Sep 20 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-8
- Add cargo test and build Requires edk2

* Fri Sep 16 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-7
- Update to StratoVirt 2.2.0-rc5

* Wed Sep 07 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-6
- Build both the gnu and musl version of stratovirt

* Tue Sep 06 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-5
- Update to StratoVirt 2.2.0-rc4

* Wed Aug 24 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-4
- Delete repeated changes to the rustflags in .cargo/config

* Tue Aug 23 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-3
- Update to StratoVirt 2.2.0-rc3
- Support VNC, usb keyboard, usb tablet, and virtio-gpu

* Sat Aug 13 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-2
- Update to StratoVirt 2.2.0-rc2
- Support migration, vhost-user net hotplug and free page reporting

* Sat Jul 30 2022 yezengruan <yezengruan@huawei.com> - 2.2.0-1
- Update to StratoVirt 2.2.0-rc1

* Wed Mar 16 2022 zhouli57 <zhouli57@huawei.com> - 2.1.0-5
- Clear some warnings.

* Sun Mar 13 2022 Jie Yang <yangjieyj.yang@huawei.com> - 2.1.0-4
- Some bug fixes.
- Eliminate build warnings and clippy errors with Rust 1.57.0.
- Update docs.

* Tue Mar 01 2022 Jie Yang <yangjieyj.yang@huawei.com> - 2.1.0-3
- Fix memory snapshot failure with hotplugged devices.
- Fix address translation for virtio devices.
- Add some test cases for microvm.
- Update some documents.

* Fri Feb 18 2022 Jie Yang <yangjieyj.yang@huawei.com> - 2.1.0-2
- Fix VFIO hotplugging failure caused by missing seccomp rules.
- Fix booting failure from disk image on x86_64.
- Update some documents.

* Fri Feb 11 2022 Jie Yang <yangjieyj.yang@huawei.com> - 2.1.0-1
- Add support for PCIe native hotplugging.
- Reduce memory consumption of reading edk2 firmware files on aarch64.
- Add support for booting from disk image directly.
- Add support for VM reboot and VM shutdown.
- Add support for guest memory preallocation.
- Add more test cases.
- Add and fix documentation.
- Replace lazy_static crate with once_cell.
- Performance enhancement for virtio-blk and virtio-net devices.
- Add ftrace support.

* Mon Aug 30 2021 Jie Yang <yangjieyj.yang@huawei.com> - 2.0.0-6
- docs: fix some statement
- machine_manager: add autodeflate for balloon
- migration: add dev_id into VirtioPciState when save and restore device state
- machine: fix more than one pcie-root-ports with same id
- migration: move function "transport_reset" to save state stage for vhost vsock device
- vfio: adding ram device region to vfio bar region
- machine_manager: move argument "serial" from -drive to -device virtio-blk-pci/device
- machine_manager: add multifunction for devices
- machine: add init_multifunction function to init multifunction
- machine: add realization for multifunction

* Tue Aug 24 2021 Ming Yang <yangming73@huawei.com> - 2.0.0-5
- add ozone in rpm package.

* Fri Aug 20 2021 Jie Yang <yangjieyj.yang@huawei.com> - 2.0.0-3
- virtio: fix dev_id initialization for virtio-pci and vfio device on aarch64 platform
- vfio: fix the problem of dma mapping failed
- syscall: add syscall "newfstatat" in x86_64-unknown-linux-gnu target
- kernel_config: update kernel config 5.10 on aarch64 platform
- machine/standard_vm: fix inappropriate file open permissions
- migration: fix an errors during the PL011 device state restore
- migration: fix an error during migration interface on aarch64
- fix spelling errors in project

* Wed Aug 18 2021 Xinle.Guo <guoxinle1@huawei.com> - 2.0.0-2
- switch stratovirt permission from 550 to 555

* Fri Aug 13 2021 Jie Yang <yangjieyj.yang@huawei.com> - 2.0.0-1
- add ACPI support
- add PCIe/PCI support
- add support for UEFI boot
- add virtio pci support
- provide standard type machine on x86_64 and aarch64
- add hot boot feature
- add VFIO support
- add initial support for being managed by libvirt

* Tue Jul 27 2021 XuFei <xufei30@huawei.com> - 0.3.0-5
- add fdt patches for musl compilation

* Wed Jun 16 2021 XuFei <xufei30@huawei.com> - 0.3.0-4
- add gcc dependency package

* Fri May 28 2021 XuFei <xufei30@huawei.com> - 0.3.0-3
- modify docs
- modify testcases
- clear clippy warnings for updating rust 1.51.0

* Sun Apr 25 2021 LiangZhang <zhangliang5@huawei.com> - 0.3.0-2
- add hydropper to package

* Sun Apr 25 2021 LiangZhang <zhangliang5@huawei.com> - 0.3.0-1
- update package to 0.3.0

* Wed Jan 13 2021 LiangZhang <zhangliang5@huawei.com> - 0.1.0-3
- update package to latest source

* Wed Jan 13 2021 XuFei <xufei30@huawei.com> - 0.1.0-2
- modify summary and description for spec

* Thu Jul 16 2020 Xu Yandong <xuyandong2@huawei.com> - 0.1.0-1
- initial package