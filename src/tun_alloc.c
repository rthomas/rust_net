#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>
#include <stdarg.h>
#include <errno.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <linux/if.h>
#include <linux/if_tun.h>
#include <sys/ioctl.h>
#include <fcntl.h>

/*
 * From Kernel documentation
 */
int tun_alloc(char *dev) {
    struct ifreq ifr;
    int fd, err;

    printf("DEVICE: %s\n", dev);

    if((fd = open("/dev/net/tun", O_RDWR)) < 0) {
        printf("Cannot open TUN/TAP dev\n");
        exit(1);
    }

    memset(&ifr, 0, sizeof(ifr));

    /* Flags: IFF_TUN   - TUN device (no Ethernet headers)
     *        IFF_TAP   - TAP device
     *
     *        IFF_NO_PI - Do not provide packet information
     */
    ifr.ifr_flags = IFF_TAP | IFF_NO_PI;

    printf("ifr_flags: %d\nTUNSETIFF: %d\n", ifr.ifr_flags, TUNSETIFF);

    if(*dev) {
        strncpy(ifr.ifr_name, dev, IFNAMSIZ);
    }

    printf("ifr: %s\n", ifr);

    if((err = ioctl(fd, TUNSETIFF, (void *) &ifr)) < 0) {
        printf("ERR: Could not ioctl tun: %s\n", strerror(errno));
        close(fd);
        return err;
    }

    strcpy(dev, ifr.ifr_name);
    return fd;
}