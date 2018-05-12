#include "helpers.h"

namespace infinityhelpers {
  namespace memory {
    void delete_RegionToken(infinity::memory::RegionToken * regionToken) {
      delete regionToken;
    }
  }
  
  namespace queues {
    void delete_QueuePair(infinity::queues::QueuePair * queuePair) {
      delete queuePair;
    }
  }
}
